use poise::serenity_prelude as serenity;

pub async fn on_error(error: poise::FrameworkError<'_, (), anyhow::Error>) {
    log::error!("Error: {:?}", error);

    if let Some(ctx) = error.ctx() {
        let result = ctx
            .send(|rep| {
                rep.embed(|emb| {
                    emb.title("Oops! something went wrong! :-(")
                        .description(format!("```{}```", error))
                        .color(serenity::Color::RED)
                })
                .ephemeral(true)
            })
            .await;
        if let Err(err) = result {
            log::error!("Error occurred when sending error message: {:?}", err);
        }
    }
}
