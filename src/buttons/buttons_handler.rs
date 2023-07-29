use crate::admin_commands::*;

pub async fn create_buttons(bot: Bot, msg: Message) -> ResponseResult<()> {
    // Create a list of buttons and send them.
    let keyboard = make_main_keyboard();
    bot.send_message(
        msg.chat.id,
        "Hola, soy un Bot que administra grupos de Telegram y seré tu \
                                 asistente personal en tu aprendizaje de Rust, \
                                 El Lenguaje de Programación\\.",
    )
    .reply_markup(keyboard)
    .await?;

    Ok(())
}

pub async fn help_action(bot: Bot, msg: Message) -> ResponseResult<()> {
    let keyboard = make_main_keyboard();
    bot.send_message(msg.chat.id, "¿Necesitas ayuda? Prueba alguna de las opciones disponibles:")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub fn make_main_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let options =
        ["Rust", "Ajustes", "Donar", "Acerca de", "Comandos", "Languages", "Ayuda", "Novedades"];

    for option in options.chunks(3) {
        let row = option
            .iter()
            .map(|&option| InlineKeyboardButton::callback(option.to_owned(), option.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

/*
fn make_row(options: &[&str]) -> Vec<InlineKeyboardButton> {
    options
        .iter()
        .map(|&option| InlineKeyboardButton::callback(option.to_owned(), option.to_owned()))
        .collect()
}

pub fn make_main_keyboard() -> InlineKeyboardMarkup {
    let keyboard = [
        make_row(&["Rust", "Ajustes", "Donar"]),
        make_row(&["Acerca de", "Comandos", "Languages"]),
        make_row(&["Ayuda", "Novedades"]),
    ];

    InlineKeyboardMarkup::new(keyboard)
}
*/

pub fn make_rust_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let rust_level = ["Principiante", "Intermedio", "Avanzado", "Volver"];

    for level in rust_level.chunks(3) {
        let row = level
            .iter()
            .map(|&level| {
                let callback_data = if level == "Volver" {
                    "back_to_main_keyboard".to_owned()
                } else {
                    level.to_owned()
                };

                InlineKeyboardButton::callback(level.to_owned(), callback_data)
            })
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}
/*
pub fn make_settings_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let settings = ["AntiSpam", "Advertencias", "Reglamento", "Volver"];

    for setting in settings.chunks(3) {
        let row = setting.iter().map(|&option| {
            match option {
                "Volver" => InlineKeyboardButton::callback(option.to_owned(), "back_to_main_keyboard".to_owned()),
                _ => InlineKeyboardButton::callback(option.to_owned(), option.to_owned())
            }
        }).collect();
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_about_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let about = ["GitHub", "Volver"];

    for option in about.chunks(3) {
        let row = option.iter().map(|&option| {
            match option {
                "Volver" => InlineKeyboardButton::callback(option.to_owned(), "back_to_main_keyboard".to_owned()),
                _ => InlineKeyboardButton::callback(option.to_owned(), option.to_owned())
            }
        }).collect();
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

 */

pub fn make_settings_keyboard() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("AntiSpam", "AntiSpam")],
        vec![InlineKeyboardButton::callback("Advertencias", "Advertencias")],
        vec![InlineKeyboardButton::callback("Reglamento", "Reglamento")],
        vec![InlineKeyboardButton::callback("Volver", "back_to_main_keyboard")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_about_keyboard() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("GitHub", "GitHub")],
        vec![InlineKeyboardButton::callback("Volver", "back_to_main_keyboard")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_donate_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let donate_options = ["PayPal", "Patreon", "Crypto", "Volver"];

    for option in donate_options.chunks(3) {
        let row = option
            .iter()
            .map(|&option| match option {
                "Volver" => InlineKeyboardButton::callback(
                    option.to_owned(),
                    "back_to_main_keyboard".to_owned(),
                ),
                _ => InlineKeyboardButton::callback(option.to_owned(), option.to_owned()),
            })
            .collect();
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_language_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let languages = ["Español", "English", "日本語", "Volver"];

    for language in languages.chunks(3) {
        let row = language
            .iter()
            .map(|&option| match option {
                "Volver" => InlineKeyboardButton::callback(
                    option.to_owned(),
                    "back_to_main_keyboard".to_owned(),
                ),
                _ => InlineKeyboardButton::callback(option.to_owned(), option.to_owned()),
            })
            .collect();
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_help_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let languages = ["Comandos", "Guía", "Soporte", "Volver"];

    for language in languages.chunks(3) {
        let row = language
            .iter()
            .map(|&option| match option {
                "Volver" => InlineKeyboardButton::callback(
                    option.to_owned(),
                    "back_to_main_keyboard".to_owned(),
                ),
                _ => InlineKeyboardButton::callback(option.to_owned(), option.to_owned()),
            })
            .collect();
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}
/*
pub fn make_help_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    keyboard.push(vec![InlineKeyboardButton::callback("Comandos".to_owned(), "Comandos".to_owned())]);
    keyboard.push(vec![InlineKeyboardButton::callback("Guía".to_owned(), "Guía".to_owned())]);
    keyboard.push(vec![InlineKeyboardButton::callback("Soporte".to_owned(), "Soporte".to_owned())]);
    keyboard.push(vec![InlineKeyboardButton::callback("Volver".to_owned(), "back_to_main_keyboard".to_owned())]);

    InlineKeyboardMarkup::new(keyboard)
}
*/

pub fn make_back_button_keyboard() -> InlineKeyboardMarkup {
    let keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    InlineKeyboardMarkup::new(keyboard)
}

pub async fn inline_query_handler(
    bot: Bot,
    q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let buttons = InlineQueryResultArticle::new(
        "0",
        "Botones",
        InputMessageContent::Text(InputMessageContentText::new("Botones")),
    )
    .reply_markup(make_main_keyboard());

    bot.answer_inline_query(q.id, vec![buttons.into()]).await?;

    Ok(())
}

pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match q.data.as_deref() {
        Some("Rust") => {
            if let Some(Message { id, chat, .. }) = q.message {
                println!("id {id:#?}");
                let text = "Selecciona tu nivel de Rust:".to_owned();
                let keyboard = make_rust_keyboard();
                bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
            }
        }

        Some("Principiante") => {
            if let Some(Message { id, chat, .. }) = q.message {
                let text = "Recomendado para Principiantes: \
                    \n\n/variables \n/constantes \n/tiposdeDatos \n/shadowing \
                    \n/operadores \n/shadowing"
                    .to_owned();

                let keyboard = make_back_button_keyboard();
                bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
            }
        }

        Some("Intermedio") => {
            if let Some(Message { id, chat, .. }) = q.message {
                let text = "Recomendado para conocimiento Intermedio: \
                    \n\n/arrays \n/tuplas \n/vectores \n/condicionales \
                    \n/loop /for /while \n/match"
                    .to_owned();

                let keyboard = make_back_button_keyboard();
                bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
            }
        }

        Some("Avanzado") => {
            if let Some(Message { id, chat, .. }) = q.message {
                let text = "Recomendado para conocimiento Avanzado: \
                    \n\n/enum \n/return \n/funciones \n/closures \
                    \n/metodos \n/option \n/struct \n/traits \
                    \n/result \n/generics \n/lifetimes \n/macros\
                    \n/ownership \n/borrowing \n/threads \n/scopes \
                    \n/async \n/referencias \n/slices \n/iterators"
                    .to_owned();

                let keyboard = make_back_button_keyboard();
                bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
            }
        }

        Some("Ajustes") => {
            if let Some(Message { id, chat, .. }) = q.message {
                let text = "Elige una opción:".to_owned();
                let keyboard = make_settings_keyboard();
                bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
            }
        }

        Some("Donar") => {
            if let Some(Message { id, chat, .. }) = q.message {
                let text = "Elige una opción:".to_owned();
                let keyboard = make_donate_keyboard();
                bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
            }
        }

        Some("Acerca de") => {
            if let Some(Message { id, chat, .. }) = q.message {
                let text = "Elige una opción:".to_owned();
                let keyboard = make_about_keyboard();
                bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
            }
        }

        Some("GitHub") => {
            if let Some(Message { id, chat, .. }) = q.message {
                let text = "GitHub: \nhttps://github.com/CrawKatt \
                    \n\nRepositorio (Estable) del Bot: \nhttps://github.com/CrawKatt/TeloxBot \
                    \n\nRepositorio (Beta) del Bot: \nhttps://github.com/CrawKatt/TeloxBeta"
                    .to_owned();
                let keyboard = make_back_button_keyboard();
                bot.edit_message_text(chat.id, id, text)
                    .reply_markup(keyboard)
                    .parse_mode(ParseMode::Html)
                    .await?;
            }
        }

        Some("Ayuda") => {
            if let Some(Message { id, chat, .. }) = q.message {
                let text = "Elige una opción:".to_owned();
                let keyboard = make_main_keyboard();
                bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
            }
        }

        Some("Languages") => {
            if let Some(Message { id, chat, .. }) = q.message {
                let text = "Elige tu idioma:".to_owned();
                let keyboard = make_language_keyboard();
                bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
            }
        }

        Some("back_to_main_keyboard") => {
            if let Some(Message { id, chat, .. }) = q.message {
                let text = "Elige una opción:".to_owned();
                let keyboard = make_main_keyboard();
                bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
            }
        }

        _ => {}
    }

    Ok(())
}
