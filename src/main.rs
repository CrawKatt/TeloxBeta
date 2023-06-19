//type Bot = DefaultParseMode<teloxide::Bot>;
// Import the commands and database modules
use crate::commands::*;
pub mod database;
pub mod commands;
pub mod utils;
pub mod buttons;

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

    // Print a message to the console to indicate that the bot has started
    println!("
╔═════════════════════════════════════════════════════╗
║                                                     ║
║      Bot Iniciado /-/ Creado por @CrawKatt /-/      ║
║                                                     ║
╚═════════════════════════════════════════════════════╝
\n");

    let bot = teloxide::Bot::from_env().parse_mode(MarkdownV2);
    let handler = dptree::entry().inspect(|u:Update| {
        println!("{u:#?}");
        //log::info!("{u:?}");
    })
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_chat_member().endpoint(chat_member_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot.clone(), handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

async fn chat_member_handler(bot:Bot, chat_member:ChatMemberUpdated) -> Result<(), Box<dyn Error + Send + Sync>> {
    let telegram_group_name = chat_member.chat.title().unwrap_or("");

    let first_name = chat_member.clone().from.first_name;
    let username = if let Some(name) = chat_member.from.username {
        name
    } else if first_name == chat_member.from.first_name {
        first_name.clone()
    } else {
        return Err("Error: Username or first name not found".into());
    };

    //let username = chat_member.from.username.unwrap();
    let member = bot.get_chat_member(chat_member.chat.id, chat_member.new_chat_member.user.id).await?;
    let member_status = member.status();

    if let ChatMemberStatus::Left = member_status {

        if username == first_name {
            bot.send_message(chat_member.chat.id, format!("Hasta pronto {username}!"))
                .parse_mode(ParseMode::Html)
                .await?;
        } else {
            bot.send_message(chat_member.chat.id, format!("Hasta pronto @{username}!"))
                .parse_mode(ParseMode::Html)
                .await?;
        }

    } else if username == first_name {
        bot.send_message(chat_member.chat.id, format!("Bienvenido a {telegram_group_name} {username}!"))
            .parse_mode(ParseMode::Html)
            .await?;
    } else {
        bot.send_message(chat_member.chat.id, format!("Bienvenido a {telegram_group_name} @{username}!"))
            .parse_mode(ParseMode::Html)
            .await?;
    }

    Ok(())
}
