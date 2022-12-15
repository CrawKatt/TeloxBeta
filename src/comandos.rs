use crate::{admin, fun, funciones};

pub use teloxide_core::types::ParseMode::MarkdownV2;
pub type MyBot = DefaultParseMode<Bot>;

use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::adaptors::DefaultParseMode;

// Derive BotCommands para analizar texto con un comando en esta enumeración.
//
// 1. `rename_rule = "lowercase"` convierte todos los comandos en letras minúsculas.
// 2. `description = "..."` especifica un texto antes de todos los comandos.
//
// Es decir, puede simplemente llamar a Command::descriptions() para obtener una descripción de
// sus comandos en este formato:
// %GENERAL-DESCRIPTION% /// %DESCRIPCIÓN GENERAL%
// %PREFIX%%COMMAND% - %DESCRIPTION% /// %PREFIJO%%COMANDO% - %DESCRIPCIÓN%
#[derive(BotCommands, Clone)]
#[command(
rename_rule = "lowercase",
description = "Hola, soy un Bot que administra grupos de Telegram y seré tu asistente personal en tu aprendizaje de Rust, El Lenguaje de Programación\\. \n\nEstos son los comandos disponibles:",
parse_with = "split"
)]

// Los comandos disponibles.
pub enum Command {
    #[command(description = "Banea a un usuario del chat\\. \n\nUso: /ban respondiendo un mensaje de un usuario\\. \n\n")]
    Ban,
    #[command(description = "Desbanea a un usuario del chat\\. \n\nUso: /unban respondiendo un mensaje de un usuario\\. \n\n")]
    Unban,
    #[command(description = "Silencia a un usuario del chat\\. \n\nUso: /mute respondiendo un mensaje de un usuario\\. \n\n")]
    Mute,
    #[command(description = "Desilencia a un usuario del chat\\. \n\nUso: /unmute respondiendo un mensaje de un usuario\\. \n\n")]
    Unmute,
    #[command(description = "Mensaje de inicio del Bot\\. \n")]
    Start,
    #[command(description = "Explica el uso de variables en Rust\\. \n")]
    Variables,
    #[command(description = "Explica el uso de constantes en Rust\\. \n")]
    Constantes,
    #[command(description = "Explica los Tipos de Datos en Rust\\. \n")]
    TiposDeDatos,
    #[command(description = "Explica el uso de los Operadores en Rust\\. \n")]
    Operadores,
    #[command(description = "Explica el uso de Arreglos/Arrays en Rust\\. \n")]
    Arrays,
    #[command(description = "Explica el uso de tuplas en Rust\\. \n")]
    Tuplas,
    #[command(description = "Explica el uso de vectores en Rust\\. \n")]
    Vectores,
    #[command(description = "Explica el uso de condicionales en Rust\\. \n")]
    Condicionales,
    #[command(description = "Explica el uso del ciclo loop en Rust\\. \n")]
    Loop,
    #[command(description = "Explica el uso del ciclo For en Rust\\. \n")]
    For,
    #[command(description = "Explica el uso del ciclo While en Rust\\. \n")]
    While,
    #[command(description = "Explica el uso de Match en Rust\\. \n")]
    Match,
    #[command(description = "Explica el uso de los Enums en Rust\\. \n")]
    Enums,
    #[command(description = "Explica el uso de Funciones en Rust\\. \n")]
    Funciones,
    #[command(description = "Explica el uso de Return en Rust\\. \n")]
    Return,
    #[command(description = "Explica el uso de Métodos en Rust\\. \n")]
    Metodos,
    #[command(description = "Explica el uso de Closures en Rust\\. \n")]
    Closures,
    #[command(description = "Explica el uso de Structs en Rust\\. \n")]
    Structs,
    #[command(description = "Explica el uso de Traits en Rust\\. \n")]
    Traits,
    #[command(description = "Explica el uso de Option en Rust\\. \n")]
    Option,
    #[command(description = "Explica el uso de Result en Rust\\. \n")]
    Result,
    #[command(description = "Explica el uso de Generics en Rust\\. \n")]
    Generics,
    #[command(description = "Explica el uso de Lifetimes en Rust\\. \n")]
    Lifetimes,
    #[command(description = "Explica el uso de Macros en Rust\\. \n")]
    Macros,
    #[command(description = "Explica el uso de Ownership en Rust\\. \n")]
    Ownership,
    #[command(description = "Explica el uso de Referencias en Rust\\. \n")]
    Referencias,
    #[command(description = "Explica el uso de Borrowing en Rust\\. \n")]
    Borrowing,
    #[command(description = "Explica el uso de los Módulos en Rust\\. \n")]
    Modulos,
    #[command(description = "Explica el Shadowing en Rust\\. \n")]
    Shadowing,
    #[command(description = "Explica el uso de los Slices en Rust\\. \n")]
    Slices,
    #[command(description = "Explica el uso de los Strings en Rust\\. \n")]
    Strings,
    #[command(description = "Explica el uso de los Iterators en Rust\\. \n")]
    Iterators,
    #[command(description = "Explica los Scopes en Rust\\. \n")]
    Scopes,
    #[command(description = "Explica el uso de los Async en Rust\\. \n")]
    Async,
    #[command(description = "Envía un Gif de Caricias de Anime\\. \n")]
    Pat,
    #[command(description = "Envía un Meme de Programación\\. \n")]
    Meme,
    #[command(description = "Envía este mensaje\\. \n")]
    Help,
    #[command(description = "Acerca de este Bot\\. \n")]
    About,
    #[command(description = "Comando para ver las novedades de la ultima versión del Bot\\. \n")]
    Novedades,
}

// Función de acción para cada comando.
pub async fn action(bot: MyBot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {

        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Start => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        // Comandos de Administración
        Command::Ban => admin::ban_user(bot, msg).await?,
        Command::Unban => admin::unban_user(bot, msg).await?,
        Command::Mute => admin::mute_user(bot, msg).await?,
        Command::Unmute => admin::unmute_user(bot, msg).await?,

        // Comandos de Información
        Command::Variables =>  funciones::variables(bot, msg).await?,
        Command::Constantes => funciones::constantes(bot, msg).await?,
        Command::TiposDeDatos => funciones::tipos_de_datos(bot, msg).await?,
        Command::Operadores => funciones::operadores(bot, msg).await?,
        Command::Arrays => funciones::arrays(bot, msg).await?,
        Command::Tuplas => funciones::tuplas(bot, msg).await?,
        Command::Vectores => funciones::vectores(bot, msg).await?,
        Command::Condicionales => funciones::condicionales(bot, msg).await?,
        Command::Loop => funciones::loops(bot, msg).await?,
        Command::For => funciones::fors(bot, msg).await?,
        Command::While => funciones::whiles(bot, msg).await?,
        Command::Match => funciones::matchs(bot, msg).await?,
        Command::Enums => funciones::enums(bot, msg).await?,
        Command::Funciones => funciones::funciones(bot, msg).await?,
        Command::Return => funciones::returns(bot, msg).await?,
        Command::Metodos => funciones::metodos(bot, msg).await?,
        Command::Closures => funciones::closures(bot, msg).await?,
        Command::Structs => funciones::estructuras(bot, msg).await?,
        Command::Traits => funciones::traits(bot, msg).await?,
        Command::Option => funciones::option(bot, msg).await?,
        Command::Result => funciones::result(bot, msg).await?,
        Command::Generics => funciones::generics(bot, msg).await?,
        Command::Lifetimes => funciones::lifetimes(bot, msg).await?,
        Command::Macros => funciones::macros(bot, msg).await?,
        Command::Ownership => funciones::ownership(bot, msg).await?,
        Command::Referencias => funciones::referencias(bot, msg).await?,
        Command::Borrowing => funciones::borrowing(bot, msg).await?,
        Command::Modulos => funciones::modulos(bot, msg).await?,
        Command::Shadowing => funciones::shadowing(bot, msg).await?,
        Command::Slices => funciones::slices(bot, msg).await?,
        Command::Strings => funciones::strings(bot, msg).await?,
        Command::Iterators => funciones::iterators(bot, msg).await?,
        Command::Scopes => funciones::scopes(bot, msg).await?,
        Command::Async => funciones::asyncs(bot, msg).await?,

        // Comandos de Diversión
        Command::Pat => fun::send_pat(bot, msg).await?, //
        Command::Meme => fun::send_random_meme(bot, msg).await?,

        // Comandos de Acerca del Bot y Novedades
        Command::About => funciones::about(bot, msg).await?,
        Command::Novedades => funciones::novedades(bot, msg).await?,
    };

    Ok(())
}

