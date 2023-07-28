# Beta de TeloxBot

En este Repositorio se encuentra el código fuente de TeloxBot, un bot de Telegram que permite la administración de grupos de Telegram a su vez que te enseña a programar en Rust mediante
Ejemplos sencillos de código.

## ¿En que se diferencia este repositorio con TeloxBot?

Esta es una versión Beta de TeloxBot, que contiene comandos y funciones experimentales que no se encuentran en la versión estable de TeloxBot.

## ¿Cómo puedo contribuir?
Puedes contribuir a este repositorio de varias formas:

***1.- Reportando errores en el código.***

***2.- Sugerencias de mejoras.***

***3.- Aportando código.***

### Documentación del código de TeloxBeta

[`main.rs`](https://github.com/CrawKatt/TeloxBeta/blob/master/src/main.rs)

```rust
// Importar los comandos y los módulos de la base de datos
use crate::commands::*;
use crate::database::*;

// Define el módulo de la base de datos
pub mod database;

// Define el módulo de los comandos
mod commands;

// Función principal que inicia el Bot
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    // Para ver los logs de Teloxide en la consola
    pretty_env_logger::init();
    log::info!("Iniciando Bot...");
    
    // Inicia las variables de entorno desde un archivo .env
    dotenv().ok();
    
    // Conexión a la base de datos
    conectar().await.expect("Error al conectar con la Base de Datos");
    
    // Imprime un mensaje en la consola para indicar que el bot ha iniciado
    println!("
╔═════════════════════════════════════════════════════╗
║                                                     ║
║      Bot Iniciado /-/ Creado por @CrawKatt /-/      ║
║                                                     ║
╚═════════════════════════════════════════════════════╝
\n");
    
    // Crea una nueva instancia de `Bot` desde las variables de entorno y establece el modo de análisis de mensajes a MarkdownV2
    let bot = teloxide::Bot::from_env().parse_mode(MarkdownV2);
    
    // Llama a la función `repl` del struct `Command` con la instancia del bot y la función `action`
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot.clone(), handler).enable_ctrlc_handler().build().dispatch().await;
    Command::repl(bot,action).await;
    Ok(())
}
```