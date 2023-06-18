use crate::admin_commands::*;

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
        let user_data = user_data.id;
        user_list += &format!("{username} [<code>{user_data}</code>] {full_name}\n");
    }

    bot.send_message(msg.chat.id, user_list).parse_mode(ParseMode::Html).await?;

    Ok(())
}

pub async fn read_database_file() -> ResponseResult<String> {
    let contents = fs::read_to_string("database.json")?;
    Ok(contents)
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

    let true = username.contains('@') else {
        let err = bot.send_message(msg.chat.id, "Debes proporcionar un username").await?;
        sleep(Duration::from_secs(5)).await;
        bot.delete_message(msg.chat.id, err.id).await?;
        println!("Debes proporcionar un username {username}");

        return Ok(());
    };

    // Obtener todos los usuarios registrados en el archivo database.json
    let contents = read_database_file().await?;
    let user_data_vec: Vec<UserData> = match serde_json::from_str(&contents) {
        Ok(vec) => vec,
        Err(e) => {
            eprintln!("Error parsing user data: {e}");
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
    // Enviar el user_id como respuesta al usuario
    match user_id {
        Some(user_id) => {
            let Ok(user_id) = user_id.parse::<u64>() else {
                println!("❌ No se pudo convertir el user_id a u64");
                return Ok(());
            };

            let Some(message) = msg.text() else {
                println!("❌ No se pudo obtener el texto del mensaje {msg:#?}");
                return Ok(()); // Si no hay un mensaje, no se hace nada
            };

            match true {
                _ if message.contains("/unban") => unban_for_testing(bot, msg.clone(), username, user_id).await?,
                _ if message.contains("/ban") => ban_for_testing(bot, msg.clone(), username, user_id).await?,
                _ if message.contains("/mute") => mute_for_testing(bot, msg.clone(), username, user_id).await?,
                _ if message.contains("/unmute") => unmute_for_testing(bot, msg.clone(), username, user_id).await?,
                _ => (),
            }
        }

        None => {
            let err = bot.send_message(msg.chat.id, format!("No se encontró ningún usuario con el username {username}"))
                .parse_mode(ParseMode::Html)
                .await?;

            sleep(Duration::from_secs(5)).await;
            bot.delete_message(msg.chat.id, err.id).await?;
            println!("No se encontró ningún usuario con el username {username}");
        }
    };

    Ok(())

    /*
        let user_id = user_data_vec.iter()
            .filter_map(|data| {
                data.username.as_ref().filter(|name| name == &username)
                    .map(|_| data.id.to_string())
            }).next();
*/
}

pub async fn unban_for_testing(bot: Bot, msg: Message, username: &str, user_id: u64) -> ResponseResult<()> {
    // Obtener información sobre el miembro del chat
    let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

    // Verificar si el usuario está baneado
    let ChatMemberStatus::Banned = chat_member.status() else {
        bot.send_message(msg.chat.id, format!("{username} [<code>{user_id}</code>] No está baneado. Usa este comando solo para remover el Ban de alguien que ya haya sido baneado"))
            .parse_mode(ParseMode::Html)
            .await?;

        return Ok(());
    };

    bot.unban_chat_member(msg.chat.id, UserId(user_id)).await?;
    let video = bot.send_video(msg.chat.id, InputFile::file("./assets/unban/1.mp4"))
        .caption(format!("{username} [<code>{user_id}</code>] desbaneado."))
        .parse_mode(ParseMode::Html).await?;

    sleep(Duration::from_secs(60)).await;
    bot.delete_message(msg.chat.id, video.id).await?;

    Ok(())
}

pub async fn ban_for_testing(bot: Bot, msg: Message, username: &str, user_id: u64) -> ResponseResult<()> {
    let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

    let ChatMemberStatus::Banned = chat_member.status() else {
        bot.ban_chat_member(msg.chat.id, UserId(user_id)).await?;
        ban_animation_generator(bot.clone(), msg.clone()).await?;

        return Ok(())
    };

    bot.send_message(msg.chat.id, format!("{username} [<code>{user_id}</code>] Ya está baneado. Usa este comando solo para banear a alguien que no haya sido baneado"))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

pub async fn mute_for_testing(bot: Bot, msg: Message, username: &str, user_id: u64) -> ResponseResult<()> {
    let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

    let ChatMemberStatus::Restricted { .. } = chat_member.status() else {
        bot.restrict_chat_member(msg.chat.id, UserId(user_id), ChatPermissions::empty()).await?;
        mute_animation_generator(bot.clone(), msg.clone()).await?;
        return Ok(())
    };

    bot.send_message(msg.chat.id, format!("{username} [<code>{user_id}</code>] Ya está silenciado. Usa este comando solo para silenciar a alguien que no haya sido silenciado"))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

pub async fn unmute_for_testing(bot: Bot, msg: Message, username: &str, user_id: u64) -> ResponseResult<()> {
    let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;
    let ChatMemberStatus::Restricted { .. } = chat_member.status() else {
        bot.send_message(msg.chat.id, format!("{username} [<code>{user_id}</code>] No está silenciado. Usa este comando solo para remover el silencio de alguien que ya haya sido silenciado"))
            .parse_mode(ParseMode::Html)
            .await?;

        return Ok(());
    };

    bot.restrict_chat_member(msg.chat.id, UserId(user_id), ChatPermissions::all()).await?;
    let ok = bot.send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4"))
        .caption(format!("{username} [<code>{user_id}</code>] Ya no está silenciado."))
        .parse_mode(ParseMode::Html).await?;

    sleep(Duration::from_secs(60)).await;
    bot.delete_message(msg.chat.id, ok.id).await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

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
        let err = bot.send_message(msg.chat.id, "❌ No has especificado un ID para obtener el usuario").await?;

        sleep(Duration::from_secs(5)).await;
        bot.delete_message(msg.chat.id, err.id).await?;
        println!("❌ No has especificado un ID para obtener el usuario {msg:#?}");

        return Ok(());
    }

    let true = arguments.contains('@') else {

        let Ok(user_id) = arguments.trim().parse::<u64>() else {
            error_message_for_user_id(bot, msg).await?;
            return Ok(())
        };

        let Some(from) = msg.from() else {
            println!("❌ No se pudo obtener el usuario que envió el mensaje {:#?}", msg);
            return Ok(());
        };

        // check if the user is an admin or owner of the chat
        let is_admin_or_owner = bot.get_chat_member(msg.chat.id, from.id).await?.is_admin_or_owner();
        // If the user is an admin or owner, ban the target user and send a ban message.
        let false = !is_admin_or_owner else {
            no_admin_err(bot, msg).await?;
            return Ok(());
        };

        let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;
        let username = chat_member.user.username.clone().unwrap_or("no username".to_string());

        let ChatMemberStatus::Banned { .. } = chat_member.status() else {
            ban_for_arguments(bot.clone(), msg.clone(), user_id, username ).await?;
            return Ok(());
        };
        already_banned(bot.clone(), msg.clone(), user_id, username).await?;

        return Ok(())
    };

    let Some(from) = msg.from() else {
        return Ok(());
    };

    let is_admin_or_owner = bot.get_chat_member(msg.chat.id, from.id).await?.is_admin_or_owner();
    let true = is_admin_or_owner else {
        no_admin_err(bot, msg).await?;
        /*
        bot.send_message(msg.chat.id, "❌ No tienes permisos para usar este comando").await?;
        sleep(Duration::from_secs(5)).await;
        bot.delete_message(msg.chat.id, msg.id).await?;
        println!("❌ No tienes permisos para usar este comando {msg:#?}");
        */
        return Ok(());
    };
    get_user_id_by_username(bot, msg).await?;

    Ok(())
}

pub async fn no_admin_err(bot: Bot, msg: Message) -> ResponseResult<()> {
    let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para usar este comando").await?;

    sleep(Duration::from_secs(5)).await;
    bot.delete_message(msg.chat.id, err.id).await?;
    bot.delete_message(msg.chat.id, msg.id).await?;
    println!("❌ No tienes permisos para usar este comando {msg:#?}");

    Ok(())
}

pub async fn already_banned(bot: Bot,msg: Message, user_id: u64, username: String) -> ResponseResult<()> {
    let err = bot.send_message(msg.chat.id, format!("❌ @{username} [<code>{user_id}</code>] ya está baneado"))
        .parse_mode(ParseMode::Html)
        .await?;

    sleep(Duration::from_secs(5)).await;
    bot.delete_message(msg.chat.id, err.id).await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn ban_for_arguments(bot:Bot, msg:Message, user_id: u64, username: String) -> ResponseResult<()> {
    bot.ban_chat_member(msg.chat.id, UserId(user_id)).await?;

    let mut rng: StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=14);

    let file_names = [
        "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif", "8.gif",
        "9.gif", "10.gif", "11.gif", "12.mp4", "13.mp4", "14.mp4",
    ];

    let get_file_name = |index: usize| -> &'static str {
        let Some(file_name) = file_names.get(index) else {
            let Some(last_file) = file_names.last() else {
                println!("❌ No se pudo obtener el último archivo de la lista de archivos {:#?}", file_names);
                return "";
            };
            return last_file;
        };
        file_name
    };

    let file_path = format!("./assets/ban/{}", get_file_name(random_number));
    match file_path.ends_with(".gif") {
        true => {
            let gif = bot.send_animation(msg.chat.id, InputFile::file(file_path))
                .caption(format!("@{username} [<code>{user_id}</code>] baneado"))
                .parse_mode(ParseMode::Html)
                .await?;

            sleep(Duration::from_secs(60)).await;
            bot.delete_message(msg.chat.id, gif.id).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        false => {
            let video = bot.send_video(msg.chat.id, InputFile::file(file_path))
                .caption(format!("@{username} [<code>{user_id}</code>] baneado"))
                .parse_mode(ParseMode::Html)
                .await?;

            sleep(Duration::from_secs(60)).await;
            bot.delete_message(msg.chat.id, video.id).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

    };

    Ok(())
}

pub async fn error_message_for_user_id(bot : Bot, msg : Message) -> ResponseResult<()> {
    let err = bot.send_message(msg.chat.id, "❌ El ID o @Username proporcionado no es válido, considera reenviar un mensaje al bot para hacer un ban por ID ERR").await?;

    sleep(Duration::from_secs(5)).await;
    bot.delete_message(msg.chat.id, err.id).await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn test_json_two(bot: Bot, msg: Message) -> ResponseResult<()> {
    let Some(user) = msg.from() else {
        println!("No user found.");
        return Ok(())
    };

    let username = match user.username {
        Some(ref username) => format!("@{username}"),
        None => user.first_name.clone(),
    };

    let user_id = user.id;
    let first_name = user.first_name.clone();
    let last_name = user.last_name.clone();

    let mut users = read_users_from_file().await;

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
            username: Some(username.clone()),
            language_code: None,
            is_premium: false,
            id: user.id,
            is_bot: false,
            first_name: new_first_name,
            last_name: new_last_name,
            added_to_attachment_menu: false,
        };
        users.push(new_user);
    } else {
        update_user_data(&mut users, user_id, username.clone(), first_name.clone(), last_name.clone());
    }

    let result = write_users_to_file(&users).await;
    match result {
        Ok(()) => println!("User saved to database."),
        Err(e) => println!("Error writing to database: {e:?}"),
    }

    match (db_username, db_first_name, db_last_name) {
        (Some(db_username), _, _) if username != db_username => {
            send_username_changed_message(bot.clone(), msg.chat.id, &user.first_name, &db_username, &username).await?;
        }

        (_, Some(db_first_name), _) if first_name != db_first_name => {
            send_first_name_changed_message(bot.clone(), msg.chat.id, &user.first_name, &db_first_name, &first_name).await?;
        }

        (_, _, Some(db_last_name)) if last_name != db_last_name => {
            send_last_name_changed_message(bot.clone(), msg.chat.id, &user.first_name, db_last_name, last_name).await?;
        }

        _ => (),
    }

    Ok(())
}

async fn read_users_from_file() -> Vec<User> {
    let json_str = match read_to_string("database.json").await {
        Ok(json_str) => json_str,
        Err(_) => String::from("[]"),
    };

    match serde_json::from_str(&json_str) {
        Ok(users) => users,
        Err(_) => vec![],
    }
}

fn update_user_data(users: &mut Vec<User>, user_id: UserId, username: String, first_name: String, last_name: Option<String>) {
    for user in users {
        if user.id == user_id {
            user.username = Some(username);
            user.first_name = first_name;
            user.last_name = last_name;
            break;
        }
    }
}

async fn write_users_to_file(users: &[User]) -> Result<(), io::Error> {
    let json_str = match serde_json::to_string_pretty(&users) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error al convertir a JSON: {e}");
            String::new()
        }
    };

    fs::write("database.json", json_str)
}

async fn send_username_changed_message(bot: Bot, chat_id: ChatId, username: &str, old_username: &str, new_username: &str) -> ResponseResult<()> {
    bot.send_message(chat_id, format!("El usuario {username} cambió su nombre de usuario de {old_username} a {new_username}."))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

async fn send_first_name_changed_message(bot: Bot, chat_id: ChatId, username: &str, old_first_name: &str, new_first_name: &str) -> ResponseResult<()> {
    bot.send_message(chat_id, format!("El usuario {username} cambió su nombre de {old_first_name} a {new_first_name}."))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

async fn send_last_name_changed_message(bot: Bot, chat_id: ChatId, username: &str, old_last_name: Option<String>, new_last_name: Option<String>) -> ResponseResult<()> {
    let old_last_name_text = match old_last_name {
        Some(last_name) => last_name,
        None => String::from("ninguno"),
    };

    let new_last_name_text = match new_last_name {
        Some(last_name) => last_name,
        None => String::from("ninguno"),
    };

    bot.send_message(chat_id, format!("El usuario {username} cambió su apellido de {old_last_name_text} a {new_last_name_text}."))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}
