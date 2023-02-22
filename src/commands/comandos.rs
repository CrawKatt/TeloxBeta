use crate::commands::*;
use crate::commands::admin_commands::*;

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
description = "Hola, soy un Bot que administra grupos de Telegram y seré tu asistente personal en tu aprendizaje de Rust, El Lenguaje de Programación\\. \
\n\nEstos son los comandos disponibles:",
parse_with = "split"
)]

// Los comandos disponibles.
// Available commands.
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
    Enum,
    #[command(description = "Explica el uso de Funciones en Rust\\. \n")]
    Funciones,
    #[command(description = "Explica el uso de Return en Rust\\. \n")]
    Return,
    #[command(description = "Explica el uso de Métodos en Rust\\. \n")]
    Metodos,
    #[command(description = "Explica el uso de Closures en Rust\\. \n")]
    Closures,
    #[command(description = "Explica el uso de Struct en Rust\\. \n")]
    Struct,
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
    String,
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
    #[command(description = "Comando para ver las novedades de la ultima versión del Bot\\. \n")]
    Novedades,
    #[command(description = "Comando para ver la información sobre un usuario\\. \n")]
    Info,

    About,

    Admin,

    User,

    Test,

    List,

}

// Función de acción para cada comando.
// Action function for each command.
pub async fn action(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {

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
        // Admin Commands
        Command::Ban => ban_user(bot, msg).await?,
        Command::Unban => unban_user(bot, msg).await?,
        Command::Mute => mute_user_admin(bot, msg).await?,
        Command::Unmute => unmute_user(bot, msg.clone()).await?,
        Command::Test => test(bot, msg).await?,
        Command::List => list(bot, msg).await?,
        Command::Info => get_chat_member(bot, msg).await?,

        // Comandos de Información
        // Info to Rust code examples Commands
        Command::Variables => ejemplos(bot, msg).await?,
        Command::Constantes => ejemplos(bot, msg).await?,
        Command::TiposDeDatos => ejemplos(bot, msg).await?,
        Command::Operadores => ejemplos(bot, msg).await?,
        Command::Arrays => ejemplos(bot, msg).await?,
        Command::Tuplas => ejemplos(bot, msg).await?,
        Command::Vectores => ejemplos(bot, msg).await?,
        Command::Condicionales => ejemplos(bot, msg).await?,
        Command::Loop => ejemplos(bot, msg).await?,
        Command::For => ejemplos(bot, msg).await?,
        Command::While => ejemplos(bot, msg).await?,
        Command::Match => ejemplos(bot, msg).await?,
        Command::Enum => ejemplos(bot, msg).await?,
        Command::Funciones => ejemplos(bot, msg).await?,
        Command::Return => ejemplos(bot, msg).await?,
        Command::Metodos => ejemplos(bot, msg).await?,
        Command::Closures => ejemplos(bot, msg).await?,
        Command::Struct => ejemplos(bot, msg).await?,
        Command::Traits => ejemplos(bot, msg).await?,
        Command::Option => ejemplos(bot, msg).await?,
        Command::Result => ejemplos(bot, msg).await?,
        Command::Generics => ejemplos(bot, msg).await?,
        Command::Lifetimes => ejemplos(bot, msg).await?,
        Command::Macros => ejemplos(bot, msg).await?,
        Command::Ownership => ejemplos(bot, msg).await?,
        Command::Referencias => ejemplos(bot, msg).await?,
        Command::Borrowing => ejemplos(bot, msg).await?,
        Command::Modulos => ejemplos(bot, msg).await?,
        Command::Shadowing => ejemplos(bot, msg).await?,
        Command::Slices => ejemplos(bot, msg).await?,
        Command::String => ejemplos(bot, msg).await?,
        Command::Iterators => ejemplos(bot, msg).await?,
        Command::Scopes => ejemplos(bot, msg).await?,
        Command::Async => ejemplos(bot, msg).await?,
        Command::Admin => get_chat_administrators(bot, msg).await?,
        Command::User => get_username(bot, msg).await?,


        // Comandos de Diversión
        // Fun Commands
        Command::Pat => send_pat(bot, msg).await?, //
        Command::Meme => send_random_meme(bot, msg).await?,

        // Comandos de Acerca del Bot y Novedades
        // About and Updates Commands
        Command::About => ejemplos(bot, msg).await?,
        Command::Novedades => ejemplos(bot, msg).await?,
    };

    Ok(())
}

