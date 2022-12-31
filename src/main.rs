mod admin;
mod comandos;
mod fun;
mod funciones;
mod database {
    pub(crate) mod db;
}

// Librería para manejar las variables de entorno
use dotenv::dotenv;

// Librería para manejar el Bot
use teloxide::{prelude::* };

// Función de arranque para el inicio del Bot mediante una Variable de Entorno.
async fn run() {
    dotenv().ok();
}

// Función principal que inicia el Bot
#[tokio::main]
async fn main() {

    dotenv().ok();
    database::db::conectar().await.expect("Error al conectar con la Base de Datos");
    pretty_env_logger::init();
    run().await;
    log::info!("Starting admin bot...");
    println!("Bot Iniciado! \n");
    let bot = Bot::from_env().parse_mode(comandos::MarkdownV2);
    comandos::Command::repl(bot, comandos::action).await;
}


