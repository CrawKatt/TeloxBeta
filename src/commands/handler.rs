use crate::commands::*;

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
    #[command(description = "Banea a un usuario del chat\\. \n\nUso: /ban respondiendo un mensaje de un usuario o a través de su ID\\. \n\n")]
    Ban,
    #[command(description = "Desbanea a un usuario del chat\\. \n\nUso: /unban respondiendo un mensaje de un usuario o a través de su ID\\. \n\n")]
    Unban,
    #[command(description = "Silencia a un usuario del chat\\. \n\nUso: /mute respondiendo un mensaje de un usuario o a través de su ID\\. \n\n")]
    Mute,
    #[command(description = "Desilencia a un usuario del chat\\. \n\nUso: /unmute respondiendo un mensaje de un usuario o a través de su ID\\. \n\n")]
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

    Testing,

    Get,

}

// Función de acción para cada comando.
// Action function for each command.
pub async fn action(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {

    match cmd {
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

        // Comandos de Acerca del Bot y Novedades
        // About and Updates Commands
        Command::About => ejemplos(bot, msg).await?,
        Command::Novedades => ejemplos(bot, msg).await?,
        _ => (),
    };

    Ok(())
}

/// Creates a keyboard made by buttons in a big column.
pub fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let options = [
        "Rust", "Ajustes", "Donar", "Acerca de",
        "Comandos", "Languages", "Ayuda", "Novedades",
    ];

    for option in options.chunks(3) {
        let row = option
            .iter()
            .map(|&option| InlineKeyboardButton::callback(option.to_owned(), option.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_buzz_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let buzz_versions = [
        "Básico", "Intermedio", "Avanzado", "Volver"
    ];

    for versions in buzz_versions.chunks(3) {
        let row = versions
            .iter()
            .map(|&version| {
                let callback_data = if version == "Volver" {
                    "back_to_main_keyboard".to_owned()
                } else {
                    version.to_owned()
                };
                InlineKeyboardButton::callback(version.to_owned(), callback_data)
            }).collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub async fn message_handler(bot: Bot, msg: Message, me: Me,) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Start) => {
                // Create a list of buttons and send them.
                let keyboard = make_keyboard();
                bot.send_message(msg.chat.id,
                                 "Hola, soy un Bot que administra grupos de Telegram y seré tu \
                                 asistente personal en tu aprendizaje de Rust, \
                                 El Lenguaje de Programación\\."
                ).reply_markup(keyboard).await?;
            }

            Ok(Command::Help) => {
                let keyboard = make_keyboard();
                bot.send_message(msg.chat.id, "¿Necesitas ayuda? Prueba alguna de las opciones disponibles:").reply_markup(keyboard).await?;
            }
            // Comandos de Administración >> Admin Commands
            Ok(Command::Ban) => ban_user(bot, msg).await?,
            Ok(Command::Unban) => unban_user(bot, msg).await?,
            Ok(Command::Mute) => mute_user_admin(bot, msg).await?,
            Ok(Command::Unmute) => unmute_user(bot, msg.clone()).await?,
            Ok(Command::List) => list_json(bot, msg).await?,
            Ok(Command::Info) => get_chat_member(bot, msg).await?,
            Ok(Command::Get) => get_user_id_by_username(bot, msg).await?,

            // Comandos de Diversión >> Fun Commands
            Ok(Command::Meme) => send_random_meme(bot, msg).await?,
            Ok(Command::Pat) => send_pat(bot, msg).await?,

            Err(_) => {
                if text.starts_with("https://t.me") {
                    bot.send_message(msg.chat.id, "Enlace Spam Detectado\\!\nAcción: Baneado").await?;
                    bot.ban_chat_member(msg.chat.id, msg.from().unwrap().id).await?;
                }

                test_json(bot, msg.clone()).await?;

                println!("{:#?}", msg);
            }

            _ => action(bot, msg, Command::Variables).await?,
        }
    }

    Ok(())
}

pub async fn inline_query_handler(bot: Bot, q: InlineQuery, ) -> Result<(), Box<dyn Error + Send + Sync>> {

    let choose_debian_version = InlineQueryResultArticle::new(
        "0",
        "Chose debian version",
        InputMessageContent::Text(InputMessageContentText::new("Debian versions:")),
    ).reply_markup(make_keyboard());

    bot.answer_inline_query(q.id, vec![choose_debian_version.into()]).await?;

    Ok(())
}

pub fn make_back_button_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let languages = ["Español", "English", "日本語","Volver"];

    for language in languages.chunks(3) {
        let row = language.iter().map(|&option| {
            match option {
                "Volver" => InlineKeyboardButton::callback(option.to_owned(), "back_to_main_keyboard".to_owned()),
                _ => InlineKeyboardButton::callback(option.to_owned(), option.to_owned())
            }
        }).collect();
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    match q.data.as_deref() {
        Some("Rust") => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    let keyboard = make_buzz_keyboard();
                    let text = "Selecciona tu nivel de Rust:".to_owned();
                    bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
                }

                _ => (),
            }
        }

        Some("back_to_main_keyboard") => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    let keyboard = make_keyboard();
                    let text = "Selecciona una Opción:".to_owned();
                    bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
                }

                _ => (),
            }
        }

        Some("Languages") => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    let text = "Próximamente".to_owned();
                    let back_keyboard = make_back_button_keyboard();
                    bot.edit_message_text(chat.id, id, text).reply_markup(back_keyboard).await?;
                }
                _ => (),
            }
        }

        Some(version) => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    let text = format!("Tu Idioma se ha configurado en: {}", version);
                    bot.edit_message_text(chat.id, id, text).await?;
                }

                _ => (),
            }
        }
        _ => {}
    }

    Ok(())
}