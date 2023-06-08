use crate::admin_commands::*;
use crate::{first_name_changed, last_name_changed};

/* ////////////||\\\\\\\\\\\\  */
/* // Experimental commands \\ */
/* \\\\\\\\\\\\||///////////// */

// Guardar Username y UserID en un archivo JSON
#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub username:   Option<String>,
    pub id:         UserId,
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
}

pub async fn read_database_file() -> ResponseResult<String> {
    let contents = fs::read_to_string("database.json")?;
    Ok(contents)
}

#[allow(unused_assignments)]
pub async fn test_json(bot : Bot, msg: Message) -> ResponseResult<()> {
    if let Some(user) = msg.from() {
        let username = match user.username {
            Some(ref username) => format!("@{username}"),
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

        let Some(user) = users.iter().find(|user| user.id == user_id) else {
            // Este bloque se ejecuta si users.iter().find(|user| user.id == user_id) es None
            return Ok(()); // Si el usuario no está registrado, se sale de la función
        };

        db_username = user.username.clone();
        db_first_name = Some(user.first_name.clone());
        db_last_name = Some(user.last_name.clone());
        is_registered = true;

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
        }

        for user in &mut users {
            if user.id == user_id {
                user.username = Some(username.clone());
                user.first_name = first_name.clone();
                user.last_name = last_name.clone();
            }
        }

        let json_str = match serde_json::to_string_pretty(&users) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Error al convertir a JSON: {e}");
                // Devolver un valor predeterminado o hacer otra cosa en caso de error
                String::new()
            }
        };

        let result = fs::write("database.json", json_str);
        match result {
            Ok(()) => println!("User saved to database."),
            Err(e) => println!("Error writing to database: {e:?}"),
        }

        let Some(db_username) = db_username else {
            return Ok(()); // Si no hay un nombre de usuario en la base de datos, no se hace nada
        };

        if username != db_username {
            bot.send_message(msg.chat.id, format!(
                "El usuario {username} cambió su nombre de usuario de {} a {}.", db_username, username))
                .parse_mode
                (ParseMode::Html)
                .await?;
            //username_changed!(bot, msg.chat.id, user, db_username, username);
        }

        let Some(db_first_name) = db_first_name else {
            return Ok(()); // Si no hay un nombre en la base de datos, no se hace nada
        };

        if first_name != db_first_name {
            first_name_changed!(bot, msg.chat.id, username, db_first_name, first_name);
        }

        let Some(db_last_name) = db_last_name else {
            return Ok(()); // Si no hay un apellido en la base de datos, no se hace nada
        };

        if last_name != db_last_name {
            last_name_changed!(bot, msg.chat.id, username, db_last_name, last_name);
        }
    }

    Ok(())
}

pub async fn testing(bot : Bot, msg : Message) -> ResponseResult<()> {

    let new_chat_members = msg.new_chat_members().unwrap();
    bot.send_message(msg.chat.id, format!("New chat members: {new_chat_members:?}")).await?;

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

        let full_name = match (user_data.first_name, user_data.last_name) {
            (Some(first_name), Some(last_name)) => format!("{first_name} {last_name}"),
            (Some(first_name), None) => first_name,
            (None, Some(last_name)) => last_name,
            (None, None) => String::from(""),
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
    let Some(text) = msg.text() else {
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

        let user_id = user_data_vec.iter()
            .find_map(|data| {
                match &data.username {
                    Some(name) if name == username => Some(data.id.to_string()),
                    _ => None,
                }
            });
/*
        let user_id = user_data_vec.iter()
            .filter_map(|data| {
                // Filtramos los elementos en base a la condición de username
                data.username.as_ref().filter(|name| name == &username)
                    // Mapeamos el resultado a un string con el ID si la condición se cumple
                    .map(|_| data.id.to_string())
            })
            // Obtenemos el primer elemento que cumple la condición, o None si no hay ninguno
            .next();
*/
        // Enviar el user_id como respuesta al usuario
        if let Some(user_id) = user_id {

            let Ok(user_id) = user_id.parse::<u64>() else {
                println!("❌ No se pudo convertir el user_id a u64");
                return Ok(());
            };

            let Some(message) = msg.text() else {
                println!("❌ No se pudo obtener el texto del mensaje {:#?}", msg);
                return Ok(()); // Si no hay un mensaje, no se hace nada
            };

            if message.contains("/unban") {
                // Obtener información sobre el miembro del chat
                let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

                // Verificar si el usuario está baneado
                let ChatMemberStatus::Banned = chat_member.status() else {
                    bot.send_message(msg.chat.id, format!(
                        "{} [<code>{}</code>] No está baneado. Usa este comando solo para remover el Ban de alguien que ya haya sido baneado",
                        username, user_id
                    )).parse_mode(ParseMode::Html).await?;
                    return Ok(());
                };

                bot.unban_chat_member(msg.chat.id, UserId(user_id)).await?;
                let video = bot.send_video(msg.chat.id, InputFile::file("./assets/unban/1.mp4"))
                    .caption(format!("{} [<code>{}</code>] desbaneado.", username, user_id))
                    .parse_mode(ParseMode::Html).await?;

                sleep(Duration::from_secs(60)).await;
                bot.delete_message(msg.chat.id, video.id).await?;

            } else if message.contains("/ban") {
                let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

                let ChatMemberStatus::Banned = chat_member.status() else {
                    bot.ban_chat_member(msg.chat.id, UserId(user_id)).await?;
                    ban_animation_generator(bot.clone(), msg.clone()).await?;

                    return Ok(())
                };

                bot.send_message(msg.chat.id, format!(
                    "{} [<code>{}</code>] Ya está baneado. Usa este comando solo para banear a alguien que no haya sido baneado",
                    username, user_id
                )).parse_mode(ParseMode::Html).await?;

            } else if message.contains("/mute") {

                let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

                let ChatMemberStatus::Restricted { .. } = chat_member.status() else {
                    bot.restrict_chat_member(msg.chat.id, UserId(user_id), ChatPermissions::empty()).await?;
                    mute_animation_generator(bot.clone(), msg.clone()).await?;
                    return Ok(())
                };

                bot.send_message(msg.chat.id, format!(
                    "{} [<code>{}</code>] Ya está silenciado. Usa este comando solo para silenciar a alguien que no haya sido silenciado",
                    username, user_id
                )).parse_mode(ParseMode::Html).await?;

            } else if message.contains("/unmute") {

                let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;
                let ChatMemberStatus::Restricted { ..} = chat_member.status() else {
                    bot.send_message(msg.chat.id, format!(
                        "{} [<code>{}</code>] No está silenciado. Usa este comando solo para remover el silencio de alguien que ya haya sido silenciado",
                        username, user_id
                    )).parse_mode(ParseMode::Html).await?;
                    return Ok(());
                };

                bot.restrict_chat_member(msg.chat.id, UserId(user_id), ChatPermissions::all()).await?;
                let ok = bot.send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4"))
                    .caption(format!("{} [<code>{}</code>] Ya no está silenciado.", username, user_id)).parse_mode(ParseMode::Html).await?;

                sleep(Duration::from_secs(60)).await;
                bot.delete_message(msg.chat.id, ok.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;

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

    let Some(text) = msg.text() else {
        return Ok(());
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

        let Some(from) = msg.from()  else {
            return Ok(());
        };

        let chat_member = bot.get_chat_member(msg.chat.id, from.id).await?;
        let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;

        let true = is_admin_or_owner else {
            bot.send_message(msg.chat.id, "❌ No tienes permisos para usar este comando").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
            println!("❌ No tienes permisos para usar este comando {:#?}", msg);
            return Ok(());
        };
        get_user_id_by_username(bot, msg).await?;

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

        let Some(from) = msg.from() else {
            println!("❌ No se pudo obtener el usuario que envió el mensaje {:#?}", msg);
            return Ok(());
        };

        let chat_member = bot.get_chat_member(msg.chat.id, from.id).await?;
        // check if the user is an admin or owner of the chat
        let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;
        // If the user is an admin or owner, ban the target user and send a ban message.

        let false = !is_admin_or_owner else {
            let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para usar este comando").await?;
            sleep(Duration::from_secs(5)).await;
            bot.delete_message(msg.chat.id, err.id).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
            return Ok(());
        };

        let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;
        let username = chat_member.user.username.clone().unwrap_or("no username".to_string());

        let ChatMemberStatus::Banned { .. } = chat_member.status() else {
            bot.ban_chat_member(msg.chat.id, UserId(user_id)).await?;
            let ban_ok = bot.send_message(msg.chat.id, format!("✅ @{} [<code>{}</code>] Baneado", username, user_id))
            .parse_mode(ParseMode::Html)
            .await?;

            sleep(Duration::from_secs(5)).await;
            bot.delete_message(msg.chat.id, ban_ok.id).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
            ban_animation_generator(bot, msg).await?;
            return Ok(());
        };

        let err = bot.send_message(msg.chat.id, format!("❌ @{} [<code>{}</code>] ya está baneado", username, user_id))
        .parse_mode(ParseMode::Html)
        .await?;
    
        sleep(Duration::from_secs(5)).await;
        bot.delete_message(msg.chat.id, err.id).await?;
        bot.delete_message(msg.chat.id, msg.id).await?;
        return Ok(());
    }

    Ok(())
}
