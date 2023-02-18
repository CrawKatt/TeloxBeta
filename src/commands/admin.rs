use crate::commands::*;

pub async fn get_chat_member(bot: MyBot, msg: Message) -> ResponseResult<()> {
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
                format!(
                    "*Nombre:* {} *Username:* @{username_user} \n*ID:*{}",
                    first_name, id_usuario
                ),
            )
            .await?;
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

pub async fn test(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let username = msg
        .from()
        .as_ref()
        .and_then(|user| user.username.as_deref())
        .unwrap_or_else(|| "sin nombre de usuario");
    let user_id = msg.from().unwrap().id;

    if let Err(_) = create_csv_file_and_add_username(&username, user_id) {
        // maneja el error
    } else {
        println!(
            "✅ Se ha añadido a: @{} con ID: [{}] al archivo CSV",
            username, user_id
        );
        bot.send_message(
            msg.chat.id,
            format!(
                "✅ Añadido: \n@{} \\[{}\\] a la Base de Datos",
                username, user_id
            ),
        )
        .await?;
    }

    Ok(())
}

pub async fn list(bot: MyBot, msg: Message) -> ResponseResult<()> {
    // Abre el archivo y lee su contenido
    let contents =
        fs::read_to_string("database.csv").unwrap_or_else(|_| "No hay registros".to_string());

    // Envía el contenido del archivo como mensaje
    bot.send_message(msg.chat.id, format!("✅ Usuarios Registrados {}", contents))
        .await?;

    Ok(())
}