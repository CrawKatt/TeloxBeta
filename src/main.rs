// Import the commands and database modules
use crate::commands::*;
use crate::database::*;

// Define a module database
pub mod database;

// Define a module commands
mod commands;

// Función principal que inicia el Bot
// Main function that starts the bot
#[tokio::main]
async fn main() {

    // Initialize the logging environment for Teloxide
    // Para ver los logs de Teloxide en la consola
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
    Command::repl(bot, action).await;
}