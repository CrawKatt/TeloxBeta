use crate::admin_commands::*;

pub fn make_main_keyboard() -> InlineKeyboardMarkup {
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

pub fn make_rust_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let rust_level = [
        "Principiante", "Intermedio", "Avanzado", "Volver"
    ];

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
            }).collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub async fn inline_query_handler(bot: Bot, q: InlineQuery, ) -> Result<(), Box<dyn Error + Send + Sync>> {

    let buttons = InlineQueryResultArticle::new(
        "0",
        "Botones",
        InputMessageContent::Text(InputMessageContentText::new("Botones")),
    ).reply_markup(make_main_keyboard());

    bot.answer_inline_query(q.id, vec![buttons.into()]).await?;

    Ok(())
}

pub fn make_donate_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let donate_options = ["PayPal", "Patreon", "Volver"];

    for option in donate_options.chunks(3) {
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

pub fn make_language_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let languages = ["Español", "English", "日本語", "Volver"];

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

pub fn make_back_button_keyboard() -> InlineKeyboardMarkup {
    let keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    InlineKeyboardMarkup::new(keyboard)
}

pub async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    match q.data.as_deref() {

        Some("Rust") | Some("Principiante") | Some("Intermedio") | Some("Avanzado") => {
            if let Some(Message { id, chat, .. }) = q.message {
                let text = match q.data.as_deref() {
                    Some("Rust") => "Selecciona tu nivel de Rust:".to_owned(),
                    Some("Principiante") => "Recomendado para Principiantes: \n\n/variables \n/constantes \n/tiposdeDatos \n/shadowing \n/operadores".to_owned(),
                    Some("Intermedio") => "Recomendado para conocimiento Intermedio: \n\n/arrays \n/tuplas \n/vectores \n/condicionales \n/loop /for /while \n/match".to_owned(),
                    Some("Avanzado") => "test 3".to_owned(),
                    _ => "".to_owned(),
                };

                let keyboard = make_rust_keyboard();
                bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
            }
        }

        Some("Ajustes") => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    let keyboard = make_back_button_keyboard();
                    let text = "Selecciona un idioma:".to_owned();
                    bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
                }

                _ => (),
            }
        }

        Some("Donar") => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    let text = "Elige como Apoyar".to_owned();
                    let back_keyboard = make_donate_keyboard();
                    bot.edit_message_text(chat.id, id, text).reply_markup(back_keyboard).await?;
                }

                _ => (),
            }
        }

        Some("Acerca de") => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    let text = "Próximamente".to_owned();
                    let back_keyboard = make_back_button_keyboard();
                    bot.edit_message_text(chat.id, id, text).reply_markup(back_keyboard).await?;
                }

                _ => (),
            }
        }

        Some("Comandos") => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    let text = "Próximamente".to_owned();
                    let back_keyboard = make_back_button_keyboard();
                    bot.edit_message_text(chat.id, id, text).reply_markup(back_keyboard).await?;
                }

                _ => (),
            }
        }

        Some("Ayuda") => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    let text = "Próximamente".to_owned();
                    let back_keyboard = make_back_button_keyboard();
                    bot.edit_message_text(chat.id, id, text).reply_markup(back_keyboard).await?;
                }

                _ => (),
            }
        }

        Some("Languages") => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    let text = "Próximamente".to_owned();
                    let back_keyboard = make_language_keyboard();
                    bot.edit_message_text(chat.id, id, text).reply_markup(back_keyboard).await?;
                }
                _ => (),
            }
        }

        Some("Novedades") => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    let text = "Novedades del Bot".to_owned();
                    let selected = make_back_button_keyboard();
                    bot.edit_message_text(chat.id, id, text).reply_markup(selected).await?;
                }

                _ => (),
            }
        }

        Some("back_to_main_keyboard") => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    let keyboard = make_main_keyboard();
                    let text = "Selecciona una Opción:".to_owned();
                    bot.edit_message_text(chat.id, id, text).reply_markup(keyboard).await?;
                }

                _ => (),
            }
        }

        Some(language) => {
            match q.message {
                Some(Message { id, chat, .. }) => {
                    #[derive(Debug, Deserialize, Serialize)]
                    pub struct Config {
                        pub language: String,
                    }

                    pub fn load_config() -> Config {
                        let contents = fs::read_to_string("config.json").expect("Failed to read config file");
                        serde_json::from_str(&contents).expect("Failed to parse config file")
                    }

                    let text = format!("Tu Idioma se ha configurado en: {}", language);
                    bot.edit_message_text(chat.id, id, text).await?;
                }

                _ => (),
            }
        }
        _ => {}
    }

    Ok(())
}