use crate::context::Context;

#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: Context<'_>) -> anyhow::Result<()> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
