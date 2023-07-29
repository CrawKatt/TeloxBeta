use teloxide::utils::html;
// Import the commands and database modules
use crate::utils::dependencies::*;
pub mod buttons;
pub mod commands;
pub mod database;
pub mod utils;

type MemberResult = Result<(), Box<dyn Error + Send + Sync>>;

struct UserIdMention {}

pub trait MentionUser {
    #[must_use]
    fn link(url: &str, text: &str) -> String {
        format!("<a href=\"{url}\">{text}</a>")
    }

    #[must_use]
    fn mention_user(user_id: UserId, text: &str) -> String {
        let url = format!("tg://user?id={user_id}");
        Self::link(&url, text)
    }
}

impl MentionUser for UserIdMention {
    fn mention_user(user_id: UserId, text: &str) -> String {
        let url = format!("tg://user?id={user_id}");
        Self::link(&url, text)
    }
}

// Main function that starts the bot
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logging environment for Teloxide
    pretty_env_logger::init();
    log::info!("Iniciando Bot...");

    // Load environment variables from a .env file
    dotenv().ok();

    // Connect to the database
    //conectar().await.expect("Error al conectar con la Base de Datos");

    // Load the init message from a txt file
    let init_message = include_str!("init_message.txt");

    // Print a message to the console to indicate that the bot has started
    println!("{init_message}\n");

    let bot = teloxide::Bot::from_env().parse_mode(MarkdownV2);
    let handler = dptree::entry()
        .inspect(|u: Update| {
            println!("{u:#?}");
        })
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler))
        .branch(
            Update::filter_chat_member()
                .filter(|m: ChatMemberUpdated| {
                    m.old_chat_member.kind.is_left() && m.new_chat_member.kind.is_present()
                })
                .endpoint(chat_member_welcome),
        )
        .branch(
            Update::filter_chat_member()
                .filter(|m: ChatMemberUpdated| {
                    m.new_chat_member.kind.is_left() && m.old_chat_member.kind.is_present()
                })
                .endpoint(chat_member_goodbye),
        );

    Dispatcher::builder(bot.clone(), handler).enable_ctrlc_handler().build().dispatch().await;

    Ok(())
}

// TODO: Mencionar por First_Name a los usuarios que se unan al grupo
async fn chat_member_welcome(bot: Bot, chat_member: ChatMemberUpdated) -> MemberResult {
    let user = chat_member.new_chat_member.user;
    let telegram_group_name = chat_member.chat.title().unwrap_or("");
    let user_id = chat_member.old_chat_member.user.id;
    let ChatId(user_id) = ChatId::from(user_id);
    let mention = user.mention().unwrap_or_else(|| html::user_mention(user_id, user.full_name().as_str()));

    bot.send_message(chat_member.chat.id, format!("bienvenido a {telegram_group_name} {mention}!"))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

async fn chat_member_goodbye(bot: Bot, chat_member: ChatMemberUpdated) -> MemberResult {
    let first_name = chat_member.old_chat_member.user.first_name;
    let username = chat_member.old_chat_member.user.username.unwrap_or_else(|| first_name.clone());
    let user_id = chat_member.old_chat_member.user.id;
    let mention = UserIdMention::mention_user(user_id, username.as_str());
    //let username = format!("<a href=\"tg://user?id={user_id}\">{username}</a>");

    bot.send_message(chat_member.chat.id, format!("Hasta pronto {mention}!"))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}
