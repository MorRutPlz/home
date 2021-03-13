use serenity::{
    client::Context,
    model::interactions::{Interaction, InteractionResponseType},
    utils::Colour,
};

use rand::{distributions::Alphanumeric, Rng};

use crate::{error, typemap::TypeMapVerificationCodes};

pub async fn execute(ctx: Context, interaction: Interaction) {
    let data = ctx.data.read().await;
    let codes = data.get::<TypeMapVerificationCodes>().unwrap();

    let username = interaction
        .data
        .as_ref()
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .options
        .get(0)
        .as_ref()
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();

    let code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    {
        let mut codes = codes.lock().await;
        let user = interaction.member.user.id;

        // Delete any existing
        let existing = codes
            .iter()
            .filter(|(_, (id, _))| *id == user)
            .map(|(a, _)| a.to_owned())
            .collect::<Vec<_>>();

        existing.into_iter().for_each(|code| {
            codes.remove(&code);
        });

        codes.insert(code.clone(), (interaction.member.user.id, username.clone()));
    }

    match interaction
        .create_interaction_response(&ctx.http, |m| {
            m.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                .interaction_response_data(|d| {
                    d.embed(|e| {
                        e.color(Colour(16748258))
                            .description(format!(
                                "Join the server ``{}.homebot.freemyip.com`` to continue",
                                code
                            ))
                            .field("Minecraft username", username, true)
                    })
                })
        })
        .await
    {
        Ok(_) => {}
        Err(e) => error!("failed to create interaction response: {}", e),
    }
}
