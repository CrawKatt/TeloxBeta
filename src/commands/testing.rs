use crate::commands::*;

/* ////////////||\\\\\\\\\\\\  */
/* // Experimental commands \\ */
/* \\\\\\\\\\\\||///////////// */

pub async fn get_chat_member(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = replied.from().unwrap();

            let chat_id = msg.chat.id;
            println!("Chat id: {}", chat_id);

            let id_usuario = user.id;
            println!("Id usuario: {}", id_usuario);

            let username_user = user.username.clone().unwrap_or_default();
            println!("Username: {}", username_user);

            let first_name = &user.first_name;
            println!("Nombre: {}", first_name);

            bot.send_message(
                msg.chat.id,
                format!("*Nombre:* {} *Username:* @{username_user} \n*ID:*{}", first_name, id_usuario), ).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "❌ No se ha respondido a ningún mensaje")
                .await?;
        }
    }

    Ok(())
}

pub fn create_csv_file_and_add_username(username: &str, user_id: UserId) -> Result<(), std::io::Error> {
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

pub async fn test(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let username = replied
                .from()
                .as_ref()
                .and_then(|user| user.username.as_deref())
                .unwrap_or("sin nombre de usuario");

            let user_id = replied.from().unwrap().id;
            bot.send_message(msg.chat.id, format!("✅ Añadido: \n@{} \\[{}\\] a la Base de Datos", username, user_id), ).await?;

            if create_csv_file_and_add_username(username, user_id).is_err() {
                // maneja el error
            } else {
                println!("✅ Se ha añadido a: @{} con ID: [{}] al archivo CSV", username, user_id);
            }
        }
        None => {
            bot.send_message(msg.chat.id, "❌ No se ha respondido a ningún mensaje").await?;
        }
    }

    Ok(())
}

pub async fn list(bot: Bot, msg: Message) -> ResponseResult<()> {
    // Abre el archivo y lee su contenido
    let contents =
        fs::read_to_string("database.csv").unwrap_or_else(|_| "No hay registros".to_string());

    // Envía el contenido del archivo como mensaje
    bot.send_message(msg.chat.id, format!("✅ Usuarios Registrados {}", contents))
        .await?;

    Ok(())
}

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