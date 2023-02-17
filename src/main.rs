use crate::commands::*;
use crate::database::*;

pub mod database;
mod commands;

// Función principal que inicia el Bot
#[tokio::main]
async fn main() {
    dotenv().ok();
    conectar().await.expect("Error al conectar con la Base de Datos");

    println!("Bot Iniciado! \n");
    let bot = Bot::from_env().parse_mode(MarkdownV2);
    Command::repl(bot, action).await;
}