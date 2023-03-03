// Import the commands and database modules
use crate::commands::*;
pub mod database;
pub mod commands;
pub mod utils;
pub mod buttons;

// Main function that starts the bot
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Initialize the logging environment for Teloxide
    pretty_env_logger::init();
    log::info!("Iniciando Bot...");

    // Load environment variables from a .env file
    dotenv().ok();

    // Connect to the database
    conectar().await.expect("Error al conectar con la Base de Datos");

    // Print a message to the console to indicate that the bot has started
    println!("
╔═════════════════════════════════════════════════════╗
║                                                     ║
║      Bot Iniciado /-/ Creado por @CrawKatt /-/      ║
║                                                     ║
╚═════════════════════════════════════════════════════╝
\n");

    // Create a new `Bot` instance from environment variables and set the message parsing mode to MarkdownV2
    let bot = teloxide::Bot::from_env().parse_mode(MarkdownV2);

    // Call the `repl` function of the `Command` struct with the bot instance and the `action` function
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot.clone(), handler).enable_ctrlc_handler().build().dispatch().await;
    Command::repl(bot,action).await;
    Ok(())
}

