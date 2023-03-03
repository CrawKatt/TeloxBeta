use crate::admin_commands::*;

/* ////////////||\\\\\\\\\\\\  */
/* // Experimental commands \\ */
/* \\\\\\\\\\\\||///////////// */

// Guardar Username y UserID en un archivo JSON
#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub username:   Option<String>,
    pub id:    UserId,
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
}

pub async fn read_database_file() -> ResponseResult<String> {
    let contents = fs::read_to_string("database.json")?;
    Ok(contents)
}

pub async fn test_json(bot : Bot, msg: Message) -> ResponseResult<()> {
    if let Some(user) = msg.from() {
        let username = match user.username {
            Some(ref username) => format!("@{}", username),
            None => user.first_name.clone(),
        };

        let user_id = user.id;
        let first_name = user.first_name.clone();
        let last_name = user.last_name.clone();

        let json_str = match read_to_string("database.json").await {
            Ok(json_str) => json_str,
            Err(_) => String::from("[]"), // Si no existe el archivo, se crea uno vacío
        };

        let users = match serde_json::from_str(&json_str) {
            Ok(u) => u,
            Err(_) => vec![],
        };

        let mut users: Vec<User> = users;
        let mut is_registered = false;
        let mut db_username = None;
        let mut db_first_name = None;
        let mut db_last_name = None;


        for user in &mut users {
            if user.id == user_id {
                db_username = user.username.clone();
                db_first_name = Some(user.first_name.clone());
                db_last_name = Some(user.last_name.clone());

                is_registered = true;
                break;
            }
        }

        if !is_registered {
            let new_first_name = first_name.clone();
            let new_last_name = last_name.clone();

            let new_user = User {
                username : Some(username.clone()),
                language_code: None,
                is_premium: false,
                id : user.id,
                is_bot: false,
                first_name : new_first_name,
                last_name : new_last_name,
                added_to_attachment_menu: false,
            };
            users.push(new_user);
        } else {
            for user in &mut users {
                if user.id == user_id {
                    user.username = Some(username.clone());
                    user.first_name = first_name.clone();
                    user.last_name = last_name.clone();
                }
            }
        }

        let json_str = match serde_json::to_string_pretty(&users) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Error al convertir a JSON: {}", e);
                // Devolver un valor predeterminado o hacer otra cosa en caso de error
                String::new()
            }
        };

        let result = fs::write("database.json", json_str);
        match result {
            Ok(()) => println!("User saved to database."),
            Err(e) => println!("Error writing to database: {:?}", e),
        }

        if let Some(db_username) = db_username {
            if username != db_username {
                bot.send_message(msg.chat.id, format!(
                    "El usuario {} cambió su nombre de usuario de {} a {}.",
                    user.first_name, db_username, username
                )).parse_mode(ParseMode::Html).await?;
            }
        }

        if let Some(db_first_name) = db_first_name {
            if first_name != db_first_name {
                bot.send_message(msg.chat.id, format!(
                    "El usuario {} cambió su nombre de [{}] a [{}].",
                    username, db_first_name, first_name
                )).parse_mode(ParseMode::Html).await?;
            }
        }

        if let Some(db_last_name) = db_last_name {
            if last_name != db_last_name {
                bot.send_message(msg.chat.id, format!(
                    "El usuario {:} cambió su apellido de [{:?}] a [{:?}].",
                    username, db_last_name, last_name
                )).parse_mode(ParseMode::Html).await?;
            }
        }
    }

    Ok(())
}

// generar una lista leyendo database.json

pub async fn list_json(bot: Bot, msg: Message) -> ResponseResult<()> {
    let user_data_vec = match get_all_users().await {
        Ok(users) => users,
        Err(_) => {
            bot.send_message(msg.chat.id, "Error al leer la base de datos.").await?;
            return Ok(());
        }
    };

    if user_data_vec.is_empty() {
        bot.send_message(msg.chat.id, "No hay usuarios registrados.").await?;
        return Ok(())
    };

    let mut user_list = String::from("Usuarios registrados: \n");

    for user_data in user_data_vec {

        let username = match user_data.username {
            Some(username) => username,
            None => String::from("sin nombre de usuario"),
        };

        let first_name = match user_data.first_name {
            Some(first_name) => first_name,
            None => String::from(""),
        };

        let last_name = match user_data.last_name {
            Some(last_name) => last_name,
            None => String::from(""),
        };

        let full_name = if last_name.is_empty() {
            first_name.clone()
        } else {
            format!("{} {}", first_name, last_name)
        };
        user_list += &format!("{} [<code>{}</code>] {}\n", username, user_data.id, full_name);
    }

    bot.send_message(msg.chat.id, user_list).parse_mode(ParseMode::Html).await?;

    Ok(())
}

pub async fn get_all_users() -> Result<Vec<UserData>, io::Error> {
    let contents = fs::read_to_string("database.json")?;
    let user_data_vec: Vec<UserData> = serde_json::from_str(&contents)?;
    Ok(user_data_vec)
}

pub async fn get_user_id_by_username(bot: Bot, msg: Message) -> ResponseResult<()> {
    // Obtener el username del mensaje
    let text = if let Some(text) = msg.text() {
        text
    } else {
        return Ok(());
    };

    let (_, username) = match text.find('@') {
        Some(index) => text.split_at(index),
        None => ("", text),
    };

    if username.contains('@') {

        // Obtener todos los usuarios registrados en el archivo database.json
        let contents = read_database_file().await?;
        let user_data_vec: Vec<UserData> = match serde_json::from_str(&contents) {
            Ok(vec) => vec,
            Err(e) => {
                eprintln!("Error parsing user data: {}", e);
                Vec::new()
            }
        };

        // Buscar el user_id correspondiente al username proporcionado
        let user_id = user_data_vec.iter()
            .find_map(|data| {
                if let Some(name) = &data.username {
                    if name == username {
                        Some(data.id.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

        // Enviar el user_id como respuesta al usuario
        if let Some(user_id) = user_id {

            let user_id = match user_id.parse::<u64>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Error parsing user_id: {}", e);
                    return Ok(());
                }
            };

            let message = if let Some(msg_text) = msg.text() {
                msg_text
            } else {
                println!("❌ No se pudo obtener el texto del mensaje {:#?}", msg);
                return Ok(())
            };

            if message.contains("/unban") {
                // Obtener información sobre el miembro del chat
                let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

                // Verificar si el usuario está baneado
                if let ChatMemberStatus::Banned = chat_member.status() {
                    // Si está baneado, desbanear al usuario
                    bot.unban_chat_member(msg.chat.id, UserId(user_id)).await?;
                    let video = bot.send_video(msg.chat.id, InputFile::file("./assets/unban/1.mp4"))
                        .caption(format!("{} [<code>{}</code>] desbaneado.", username, user_id)).parse_mode(ParseMode::Html).await?;

                    sleep(Duration::from_secs(60)).await;
                    bot.delete_message(msg.chat.id, video.id).await?;
                } else {
                    // Si no está baneado, enviar un mensaje de error
                    bot.send_message(msg.chat.id, format!(
                        "{} [<code>{}</code>] No está baneado. Usa este comando solo para remover el Ban de alguien que ya haya sido baneado",
                        username,
                        user_id
                    )).parse_mode(ParseMode::Html).await?;
                }

            } else if message.contains("/ban") {

                let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;
                if let ChatMemberStatus::Banned = chat_member.status() {

                    bot.send_message(msg.chat.id, format!(
                        "{} [<code>{}</code>] Ya está baneado. Usa este comando solo para banear a alguien que no haya sido baneado",
                        username,
                        user_id
                    )).parse_mode(ParseMode::Html).await?;

                } else {
                    bot.ban_chat_member(msg.chat.id, UserId(user_id)).await?;
                    ban_animation_generator(bot.clone(), msg.clone()).await?;
                }

                bot.ban_chat_member(msg.chat.id, UserId(user_id)).await?;
                ban_animation_generator(bot, msg).await?;

            } else if message.contains("/mute") {

                let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;
                if let ChatMemberStatus::Restricted { ..} = chat_member.status() {
                    bot.send_message(msg.chat.id, format!(
                        "{} [<code>{}</code>] Ya está silenciado. Usa este comando solo para silenciar a alguien que no haya sido silenciado",
                        username,
                        user_id
                    )).parse_mode(ParseMode::Html).await?;
                } else {
                    bot.restrict_chat_member(msg.chat.id, UserId(user_id), ChatPermissions::empty()).await?;
                    mute_animation_generator(bot.clone(), msg.clone()).await?;
                }

            } else if message.contains("/unmute") {

                let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;
                if let ChatMemberStatus::Restricted { .. } = chat_member.status() {
                    bot.restrict_chat_member(msg.chat.id, UserId(user_id), ChatPermissions::all()).await?;
                    let ok = bot.send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4"))
                        .caption(format!("{} [<code>{}</code>] Ya no está silenciado.", username, user_id)).parse_mode(ParseMode::Html).await?;

                    sleep(Duration::from_secs(60)).await;
                    bot.delete_message(msg.chat.id, ok.id).await?;
                    bot.delete_message(msg.chat.id, msg.id).await?;
                } else {
                    bot.send_message(msg.chat.id, format!(
                        "{} [<code>{}</code>] No está silenciado. Usa este comando solo para remover el silencio de alguien que ya haya sido silenciado",
                        username,
                        user_id
                    )).parse_mode(ParseMode::Html).await?;
                }

            } else {
                bot.send_message(msg.chat.id, format!("{} [<code>{}</code>]", username, user_id)).parse_mode(ParseMode::Html).await?;
            }

        } else {
            let err = bot.send_message(msg.chat.id, format!("No se encontró ningún usuario con el username {}", username)).parse_mode(ParseMode::Html).await?;
            bot.delete_message(msg.chat.id, err.id).await?;
            println!("No se encontró ningún usuario con el username {}", username);
        }

    } else {
        let err = bot.send_message(msg.chat.id, "Debes proporcionar un username").await?;
        sleep(Duration::from_secs(5)).await;
        bot.delete_message(msg.chat.id, err.id).await?;
        println!("Debes proporcionar un username {}", username);
    }

    Ok(())
}

// ban por UserID
pub async fn get_user_id_by_arguments(bot: Bot, msg: Message) -> ResponseResult<()> {
    // extract the text content of the message
    let text = if let Some(msg_text) = msg.text() {
        msg_text
    } else {
        println!("❌ No se pudo obtener el texto del mensaje {:#?}", msg);
        return Ok(())
    };

    // get the arguments after the command trigger
    let (_, arguments) = match text.find(' ') {
        Some(index) => text.split_at(index),
        None => ("", text),
    };

    // check if the arguments are empty
    if arguments.is_empty() {
        bot.send_message(msg.chat.id, "❌ No has especificado un ID para obtener el usuario").await?;
        bot.delete_message(msg.chat.id, msg.id).await?;
        println!("❌ No has especificado un ID para obtener el usuario {:#?}", msg);

        return Ok(());
    }

    // if arguments is String, then use this
    if arguments.contains('@') {

        if let Some(from) = msg.from() {

            let chat_member = bot.get_chat_member(msg.chat.id, from.id).await?;
            let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;

            if is_admin_or_owner {
                get_user_id_by_username(bot, msg).await?;

            } else {
                let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para usar este comando").await?;
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, err.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
                return Ok(());
            }
        }

    } else {
        // extract the user ID from the arguments
        let user_id = match arguments.trim().parse::<u64>() {
            Ok(id) => id,
            Err(_) => {
                let err = bot.send_message(msg.chat.id, "❌ El ID o @Username proporcionado no es válido, considera reenviar un mensaje al bot para hacer un ban por ID").await?;
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, err.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;

                return Ok(());
            }
        };

        // get information about the user who sent the message
        if let Some(from) = msg.from() {
            let chat_member = bot.get_chat_member(msg.chat.id, from.id).await?;
            // check if the user is an admin or owner of the chat
            let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;
            // If the user is an admin or owner, ban the target user and send a ban message.
            if is_admin_or_owner {
                // ban the target user
                bot.ban_chat_member(msg.chat.id, UserId(user_id)).await?;

                // send a ban message with the target user's ID
                let ban_ok = bot.send_message(msg.chat.id, format!("✅ Baneado \\[`{}`\\]", user_id)).await?;

                // delete the command message
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, ban_ok.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;

                // Choose a random ban animation to send.
                ban_animation_generator(bot, msg).await?;

                // If the user is not an admin or owner, delete the command message and send an error message.
            } else {
                // send an error message
                let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para banear a un usuario").await?;

                // delete the command message and the error message
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, err.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
            };
        } else {
            println!("No se pudo obtener la información del usuario que envió el mensaje");
        }
        // println!("JSON Info: {:#?}", chat_member);
    }

    Ok(())
}

