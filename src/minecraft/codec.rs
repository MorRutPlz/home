use feather_protocol::{
    packets::server::EncryptionRequest, ClientLoginPacket, MinecraftCodec as FeatherCodec,
    Readable, ServerLoginPacket, Writeable,
};
use rand_core::OsRng;
use rsa::{PaddingScheme, PublicKeyParts, RSAPrivateKey};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::timeout,
};

pub struct MinecraftCodec {
    inner: TcpStream,
    inner_codec: FeatherCodec,
    rsa_key: RSAPrivateKey,
    pub rsa_public_key: Vec<u8>,
}

impl MinecraftCodec {
    pub fn new(stream: TcpStream) -> MinecraftCodec {
        let rsa_key = RSAPrivateKey::new(&mut OsRng, 1024).unwrap();
        let rsa_public_key =
            rsa_der::public_key_to_der(&rsa_key.n().to_bytes_be(), &rsa_key.e().to_bytes_be());

        MinecraftCodec {
            inner: stream,
            inner_codec: FeatherCodec::new(),
            rsa_key,
            rsa_public_key,
        }
    }

    pub async fn enable_encryption(&mut self) -> Result<[u8; 16], &'static str> {
        let verify_token: [u8; 16] = rand::random();
        let request = EncryptionRequest {
            server_id: String::new(), // always empty
            public_key: self.rsa_public_key.clone(),
            verify_token: verify_token.to_vec(),
        };

        self.send(&ServerLoginPacket::EncryptionRequest(request))
            .await?;

        let response = match self.next::<ClientLoginPacket>().await? {
            ClientLoginPacket::EncryptionResponse(n) => n,
            _ => return Err("expected encryption response"),
        };

        let shared_secret = self
            .rsa_key
            .decrypt(PaddingScheme::PKCS1v15Encrypt, &response.shared_secret)
            .map_err(|_| "failed to decrypt shared secret")?;

        let received_verify_token = self
            .rsa_key
            .decrypt(PaddingScheme::PKCS1v15Encrypt, &response.verify_token)
            .map_err(|_| "failed to decrypt verify token")?;

        if received_verify_token != verify_token {
            return Err("verify tokens don't match");
        }

        let shared_secret = match shared_secret.len() {
            n if n == 16 => {
                let mut ss = [0u8; 16];
                ss.clone_from_slice(&shared_secret);
                ss
            }
            _ => return Err("shared secret not 16 bytes"),
        };

        self.inner_codec.enable_encryption(shared_secret);

        Ok(shared_secret)
    }

    pub async fn next<P: Readable>(&mut self) -> Result<P, &'static str> {
        loop {
            match self.inner_codec.next_packet::<P>() {
                Ok(Some(n)) => return Ok(n),
                Ok(_) => {}
                Err(_) => return Err("failed to get next packet (codec error)"),
            }

            let mut buffer = [0u8; 512];
            let read = timeout(Duration::from_secs(10), self.inner.read(&mut buffer))
                .await
                .map_err(|_| "timeout reached before client sent any data")
                .map(|x| x.map_err(|_| "io error while reading from socket"))??;

            if read == 0 {
                return Err("read 0 bytes");
            }

            self.inner_codec.accept(&buffer[..read]);
        }
    }

    pub async fn send<P: Writeable>(&mut self, packet: P) -> Result<(), &'static str> {
        let mut output = Vec::new();
        self.inner_codec.encode(&packet, &mut output);

        match self.inner.write_all(&output).await {
            Ok(_) => Ok(()),
            Err(_) => Err("io error while writing to socket"),
        }
    }
}
