use crate::commands::*;
/* ////////////||\\\\\\\\\\\\  */
/* // Experimental commands \\ */
/* \\\\\\\\\\\\||///////////// */

pub async fn get_chat_member(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = replied.from().unwrap();
            let id_usuario = user.id;
            let username_user = user.username.clone().unwrap_or_default();
            let first_name = &user.first_name;
            bot.send_message(msg.chat.id, format!("<b>Nombre:</b> {} \n<b>Username:</b> @{:} \n<b>ID: </b><code>{}</code>", first_name, username_user, id_usuario)).parse_mode(ParseMode::Html).await?;
        }

        None => {
            bot.send_message(msg.chat.id, "❌ No se ha respondido a ningún mensaje").await?;
        }
    }

    Ok(())
}
/*
pub fn create_csv_file_and_add_username(username: &str, user_id: UserId) -> Result<(), io::Error> {
    let contents = fs::read_to_string("database.csv")?;
    if !contents.contains(username) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("database.csv")?;
        file.write_all(format!("@{},{}\n", username, user_id).as_bytes())?;
    }
    Ok(())
}
*/

/*
pub async fn test(_: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let username = replied
                .from()
                .as_ref()
                .and_then(|user| user.username.as_deref())
                .unwrap_or("sin nombre de usuario");

            let user_id = replied.from().unwrap().id;

            if create_csv_file_and_add_username(username, user_id).is_err() {
                // maneja el error
            } else {
                println!("✅ Añadido: \n@{} \\[{}\\] a la Base de Datos", username, user_id);
            }

        } None => {
            let username = msg
                .from()
                .as_ref()
                .and_then(|user| user.username.as_deref())
                .unwrap_or("sin nombre de usuario");
            let user_id = msg.from().unwrap().id;

            if create_csv_file_and_add_username(username, user_id).is_err() {
                println!("Error al añadir a la Base de Datos");
            } else {
                println!("Base de Datos: @{:#?}, {:#?}", username, user_id);
            }
        }
    }

    Ok(())
}
*/
/*
pub async fn list(bot: Bot, msg: Message) -> ResponseResult<()> {
    // Abre el archivo y lee su contenido
    let contents = fs::read_to_string("database.csv").unwrap_or_else(|_| "No hay registros".to_string());

    // Envía el contenido del archivo como mensaje
    bot.send_message(msg.chat.id, format!("✅ Usuarios Registrados {}", contents)).await?;

    Ok(())
}
*/

// get chat administrators and async function
pub async fn get_chat_administrators(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_administrators = bot.get_chat_administrators(msg.chat.id).await?;
    println!("{:#?}", chat_administrators);

    bot.send_message(msg.chat.id, format!("Chat Administrators: `{:#?}`", chat_administrators)).await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

// get @username from message from command and async function
pub async fn get_username(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = replied.from().unwrap();

            let username_user = user.clone().username;
            println!("@username : {:?}", username_user);

            bot.send_message(msg.chat.id, format!("Tu @Username es : @{}", username_user.unwrap())).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        None => {
            bot.send_message(msg.chat.id, "No has respondido a ningún mensaje").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
    }

    Ok(())
}

// Guardar Username y UserID en un archivo JSON

#[derive(Serialize, Deserialize)]
pub struct UserData {
    username: Option<String>,
    user_id: UserId,
}

pub fn create_json_file_and_add_username(username: &str, user_id: UserId) -> Result<(), io::Error> {
    let contents = fs::read_to_string("database.json")?;
    let mut user_data_vec: Vec<UserData> = serde_json::from_str(&contents).unwrap_or_default();

    if !user_data_vec.iter().any(|data| data.username == Some(username.to_string())) {
        let user_data = UserData {
            username: Some(username.to_owned()),
            user_id,
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

pub async fn test_json(_: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let username = replied
                .from()
                .as_ref()
                .and_then(|user| user.username.as_deref())
                .map(|username| format!("@{}", username))
                .unwrap_or("sin nombre de usuario".to_owned());

            let user_id = replied.from().unwrap().id;

            if create_json_file_and_add_username(&username, user_id).is_err() {
                // maneja el error
            } else {
                println!("✅ Añadido: \n{} \\[{}\\] a la Base de Datos", username, user_id);
            }

        }

        None => {
            let username = msg
                .from()
                .as_ref()
                .and_then(|user| user.username.as_deref())
                .unwrap_or("sin nombre de usuario");

            let user_id = msg.from().unwrap().id;

            if create_json_file_and_add_username(&format!("@{}", username), user_id).is_err() {
                println!("Error al añadir a la Base de Datos");
            } else {
                println!("Base de Datos: {:}, {:}", username, user_id);
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
        let username = user_data.username.unwrap_or_else(|| String::from("sin nombre de usuario"));
        user_list += &format!("@{} \\[{}\\]\n", username, user_data.user_id);
    }

    bot.send_message(msg.chat.id, user_list).await?;

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
            bot.send_message(msg.chat.id, format!("El user_id de {} es <code>{}</code>", username, id)).parse_mode(ParseMode::Html).await?;
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