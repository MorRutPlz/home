use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TTSResponse {
    pub audio_content: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TTSRequest {
    pub audio_config: AudioConfig,
    pub input: SynthesisInput,
    pub voice: VoiceSelectionParams,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceSelectionParams {
    pub language_code: String,
    pub name: Option<String>,
    pub ssml_gender: Option<SsmlVoiceGender>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SsmlVoiceGender {
    Female,
    Male,
    Neutral,
    SsmlVoiceGenderUnspecified,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SynthesisInput {
    Text(String),
    Ssml(String),
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioConfig {
    pub audio_encoding: AudioEncoding,
    pub speaking_rate: Option<f64>,
    pub pitch: Option<f64>,
    pub volume_gain_db: Option<f64>,
    pub sample_rate_hertz: Option<u32>,
    pub effects_profile_id: Option<Vec<AudioProfile>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudioEncoding {
    AudioEncodingUnspecified,
    Linear16,
    #[serde(rename = "MP3")]
    MP3,
    OggOpus,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AudioProfile {
    HandsetClassDevice,
    HeadphoneClassDevice,
    LargeAutomotiveClassDevice,
    LargeHomeEntertainmentClassDevice,
    MediumBluetoothSpeakerClassDevice,
    SmallBluetoothSpeakerClassDevice,
    TelephonyClassApplication,
    WearableClassDevice,
}
