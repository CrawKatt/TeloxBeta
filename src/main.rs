//use teloxide::utils::html;
// Import the commands and database modules
use crate::utils::dependencies::*;
pub mod database;
pub mod commands;
pub mod utils;
pub mod buttons;

type MemberResult = Result<(), Box<dyn Error + Send + Sync>>;

// Custom Unwrap method for Option<T>
pub trait UnwrapOrErr<T> {
    fn unwrap_or_exit(self, msg: &str) -> ResponseResult<T>;
}

#[derive(Debug)]
pub struct ResponseError {
    pub message: String,
}

impl ResponseError {
    fn new(message: &str) -> ResponseError {
        ResponseError {
            message: message.to_string(),
        }
    }
}

type ResponseResult<T> = Result<T, ResponseError>;

impl<T: Default> UnwrapOrErr<T> for Option<T> {
    fn unwrap_or_exit(self, msg: &str) -> ResponseResult<T> {
        match self {
            Some(val) => Ok(val),
            None => {
                eprintln!("{}", msg);
                Err(ResponseError::new("Error"))
            }
        }
    }
}
// End Custom Unwrap method for Option<T>

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
    let handler = dptree::entry().inspect(|u:Update| {
        println!("{u:#?}");
    })
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler))
        .branch(Update::filter_chat_member()
            .filter(|m: ChatMemberUpdated|
                m.old_chat_member.kind.is_left() &&
                    m.new_chat_member.kind.is_present())
            .endpoint(chat_member_welcome))
        .branch(Update::filter_chat_member()
            .filter(|m: ChatMemberUpdated|
                m.new_chat_member.kind.is_left() &&
                    m.old_chat_member.kind.is_present())
            .endpoint(chat_member_goodbye));

    Dispatcher::builder(bot.clone(), handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

// TODO: Mencionar por First_Name a los usuarios que se unan al grupo
async fn chat_member_welcome(bot:Bot, chat_member:ChatMemberUpdated) -> MemberResult {
    let telegram_group_name = chat_member.chat.title().unwrap_or("");
    let first_name = chat_member.old_chat_member.user.first_name;
    let username = chat_member.old_chat_member.user.username.unwrap_or(first_name.clone());
    //let url = chat_member.old_chat_member.user.url();
    //let test_name= chat_member.old_chat_member.user.mention();

    bot.send_message(chat_member.chat.id, format!("bienvenido a {telegram_group_name} {username}!"))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

async fn chat_member_goodbye(bot:Bot, chat_member:ChatMemberUpdated) -> MemberResult {
    let first_name = chat_member.old_chat_member.user.first_name;
    let username = chat_member.old_chat_member.user.username.unwrap_or(first_name.clone());

    bot.send_message(chat_member.chat.id, format!("Hasta pronto {username}!"))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}