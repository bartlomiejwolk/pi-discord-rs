use super::SlashCommand;
use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommandOption, EditInteractionResponse,
};

pub struct AutoThreadCommand;

#[async_trait]
impl SlashCommand for AutoThreadCommand {
    fn name(&self) -> &'static str {
        "auto_thread"
    }

    fn description(&self, i18n: &crate::i18n::I18n) -> String {
        i18n.get("cmd_auto_thread_desc")
    }

    fn options(&self, i18n: &crate::i18n::I18n) -> Vec<CreateCommandOption> {
        vec![CreateCommandOption::new(
            CommandOptionType::Boolean,
            "enable",
            i18n.get("cmd_auto_thread_opt_enabled"),
        )
        .required(true)]
    }

    async fn execute(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        state: &crate::AppState,
    ) -> anyhow::Result<()> {
        command.defer_ephemeral(&ctx.http).await?;

        // Refuse if invoked inside a thread — auto-thread applies to parent channels only
        if let Ok(channel) = command.channel_id.to_channel(&ctx.http).await {
            if let Some(guild_ch) = channel.guild() {
                if matches!(
                    guild_ch.kind,
                    serenity::model::channel::ChannelType::PublicThread
                        | serenity::model::channel::ChannelType::PrivateThread
                        | serenity::model::channel::ChannelType::NewsThread
                ) {
                    let i18n = state.i18n.read().await;
                    let msg = i18n.get("auto_thread_not_in_thread");
                    command
                        .edit_response(&ctx.http,
                            EditInteractionResponse::new().content(msg),
                        )
                        .await?;
                    return Ok(());
                }
            }
        }

        let enable = command
            .data
            .options
            .iter()
            .find(|o| o.name == "enable")
            .and_then(|o| o.value.as_bool())
            .unwrap_or(true);

        let ch_id = command.channel_id.to_string();

        let mut channel_config = crate::commands::agent::ChannelConfig::load()
            .await
            .unwrap_or_default();
        channel_config.set_auto_thread(&ch_id, enable);
        channel_config.save().await?;

        let i18n = state.i18n.read().await;
        let msg = i18n.get(if enable {
            "auto_thread_on"
        } else {
            "auto_thread_off"
        });

        command
            .edit_response(&ctx.http, EditInteractionResponse::new().content(msg))
            .await?;

        Ok(())
    }
}
