use macro_env::{dotenvreader, input, macro_env, systemreader};
use poise::serenity_prelude::{self as serenity, futures::TryFutureExt};
use tokio_schedule::{Job, every};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[allow(clippy::unreadable_literal)]
static MAIN_GUILD: serenity::GuildId = serenity::GuildId::new(771430766529085440);
#[allow(clippy::unreadable_literal)]
static BLEND_CHANNEL: serenity::ChannelId = serenity::ChannelId::new(813469691993391104);

static BLEND_WEBHOOK: &str = "";

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at wow {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

fn setup_called_every_day(http: serenity::Context) {
    let thingy = every(1)
        .day()
        .at(18, 43, 10)
        .in_timezone(&chrono_tz::US::Eastern)
        .perform(move || called_every_day(http.clone()).map_ok_or_else(|_| (), |()| ()));
    tokio::spawn(thingy);
}

async fn called_every_day(http: impl serenity::CacheHttp) -> Result<(), Error> {
    let webhook = serenity::Webhook::from_url(&http.http(), BLEND_WEBHOOK).await?;
    let builder = serenity::ExecuteWebhook::new().content("asdf");
    let _ = webhook.execute(&http, true, builder).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = macro_env!("DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, MAIN_GUILD)
                    .await?;
                setup_called_every_day(ctx.clone());

                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    #[allow(clippy::unwrap_used)]
    client.unwrap().start().await.unwrap();
}
