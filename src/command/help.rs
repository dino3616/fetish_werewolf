use crate::context::Context;

/// Shows help message.
#[poise::command(prefix_command, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> anyhow::Result<()> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is a Discord Bot for playing fetish werewolf.",
            show_context_menu_commands: true,
            ..Default::default()
        },
    )
    .await?;

    Ok(())
}
