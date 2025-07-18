use crate::{Context, Error};

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// 
/// Plays music 
///
/// Joins a voice channel and plays music based on link or search query
/// 
/// ```
/// ~play Props & Mayhem Pierce The Veil
/// ~play https://open.spotify.com/track/2SYMnBpx7zyhSDPQwoOlFO?si=99024f1b56d2490a
/// ~play https://youtu.be/yBbr9n9JnZg?si=Rw1RRwLftmeGreOx
/// ```
#[poise::command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Command to play music"] choice: String,
) -> Result<(), Error> {

    let response = format!("You want to play {choice}. {choice} has now started playing!");
    ctx.say(response).await?;

    Ok(())
}