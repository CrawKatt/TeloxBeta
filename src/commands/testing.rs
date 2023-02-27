use crate::admin_commands::*;

/* ////////////||\\\\\\\\\\\\  */
/* // Experimental commands \\ */
/* \\\\\\\\\\\\||///////////// */

// Guardar Username y UserID en un archivo JSON
#[derive(Serialize, Deserialize)]
pub struct UserData {
    username:   Option<String>,
    user_id:    UserId,
    first_name: Option<String>,
    last_name:  Option<String>,
}

pub fn create_json_file_and_add_username(
    username:   &str,
    user_id:    UserId,
    first_name: Option<String>,
    last_name:  Option<String>,

) -> Result<(), io::Error> {
    let contents = fs::read_to_string("database.json")?;
    let mut user_data_vec: Vec<UserData> = serde_json::from_str(&contents).unwrap_or_default();

    if !user_data_vec.iter().any(|data| data.username == Some(username.to_string())) {
        let user_data = UserData {
            username: Some(username.to_owned()),
            user_id,
            first_name,
            last_name,
        };
        user_data_vec.push(user_data);

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("database.json")?;

        let json_data = serde_json::to_string_pretty(&user_data_vec)?;
        let mut writer = io::BufWriter::new(file);
        writer.write_all(json_data.as_bytes())?;
    }

    Ok(())
}

pub async fn test_json(msg: Message) -> ResponseResult<()> {
    if let Some(user) = msg.from() {
        let username = match user.username {
            Some(ref username) => format!("@{}", username),
            None => user.first_name.clone(),
        };
        let user_id = user.id;
        let first_name = user.first_name.clone();
        let last_name = user.last_name.clone();

        if create_json_file_and_add_username(&username, user_id, Some(first_name), last_name).is_err() {
            println!("Error al añadir a la Base de Datos");
        } else {
            println!("Base de Datos: {:}, {:}", username, user_id);
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
        let username = user_data.username.unwrap_or_else(|| String::from("sin nombre de usuario"));
        let first_name = user_data.first_name.unwrap_or_else(|| String::from(""));
        let last_name = user_data.last_name.unwrap_or_else(|| String::from(""));
        let full_name = if last_name.is_empty() {
            first_name.clone()
        } else {
            format!("{} {}", first_name, last_name)
        };
        user_list += &format!("{} [<code>{}</code>] {}\n", username, user_data.user_id, full_name);
    }

    bot.send_message(msg.chat.id, user_list).parse_mode(ParseMode::Html).await?;

    Ok(())
}

pub async fn get_all_users() -> Result<Vec<UserData>, io::Error> {
    let contents = fs::read_to_string("database.json")?;
    let user_data_vec: Vec<UserData> = serde_json::from_str(&contents).unwrap_or_default();
    Ok(user_data_vec)
}

pub async fn get_user_id_by_username(bot: Bot, msg: Message) -> ResponseResult<()> {
    // Obtener el username del mensaje
    let text = &msg.text().unwrap();
    let (_, username) = text.split_at(text.find('@').unwrap_or(text.len()));

    if username.contains('@') {

        // Obtener todos los usuarios registrados en el archivo database.json
        let contents = fs::read_to_string("database.json").unwrap_or_default();
        let user_data_vec: Vec<UserData> = serde_json::from_str(&contents).unwrap_or_default();

        // Buscar el user_id correspondiente al username proporcionado
        let user_id = user_data_vec.iter()
            .find_map(|data| {
                if let Some(name) = &data.username {
                    if name == username {
                        Some(data.user_id.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

        // Enviar el user_id como respuesta al usuario
        if let Some(id) = user_id {
            let id = id.parse::<u64>().unwrap();
            bot.ban_chat_member(msg.chat.id, UserId(id)).await?;
            bot.send_message(msg.chat.id, format!("{} [<code>{}</code>] baneado.", username, id)).parse_mode(ParseMode::Html).await?;
            ban_animation_generator(bot, msg).await?;

        } else {
            bot.send_message(msg.chat.id, format!("No se encontró ningún usuario con el username {}", username)).await?;
            println!("No se encontró ningún usuario con el username {}", username);
        }

    } else {
        bot.send_message(msg.chat.id, "Debes proporcionar un username").await?;
        println!("Debes proporcionar un username {}", username);
    }

    Ok(())
}



// ban por UserID
pub async fn get_user_id_by_arguments(bot: Bot, msg: Message) -> ResponseResult<()> {
    // extract the text content of the message
    let text = &msg.text().unwrap();

    // get the arguments after the command trigger
    let (_, arguments) = text.split_at(text.find(' ').unwrap_or(text.len()));

    // check if the arguments are empty
    if arguments.is_empty() {
        bot.send_message(msg.chat.id, "❌ No has especificado un ID para obtener el usuario").await?;
        bot.delete_message(msg.chat.id, msg.id).await?;
        println!("❌ No has especificado un ID para obtener el usuario {:#?}", msg);
        return Ok(());
    }
    // if arguments is String, then use this
    if arguments.contains('@') {
        let chat_id = msg.chat.id;
        let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
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

    } else {
        // extract the user ID from the arguments
        let user_id = match arguments.trim().parse::<u64>() {
            Ok(id) => id,
            Err(_) => {
                bot.send_message(msg.chat.id, "❌ El ID proporcionado no es válido").await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
                return Ok(());
            }
        };

        // get the ID of the chat where the message was sent
        let chat_id = msg.chat.id;

        // get information about the user who sent the message
        let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
        // println!("JSON Info: {:#?}", chat_member);

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
            let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para banear a un usuario", ).await?;

            // delete the command message and the error message
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, err.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
        };
    }

    Ok(())
}