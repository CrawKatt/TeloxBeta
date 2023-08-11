use crate::dependencies::*;

// ////////////||\\\\\\\\\\\\
// // Experimental commands \\
// \\\\\\\\\\\\||/////////////

// Guardar Username y UserID en un archivo JSON
#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub username: Option<String>,
    pub id: UserId,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

/// # Errors
pub async fn list_json(bot: Bot, msg: Message) -> ResponseResult<()> {
    let Ok(user_data_vec) = get_all_users() else {
        bot.send_message(msg.chat.id, "Error al leer la base de datos.")
            .await?;

        return Ok(())
    };

    if user_data_vec.is_empty() {
        bot.send_message(msg.chat.id, "No hay usuarios registrados.")
            .await?;

        return Ok(());
    };

    let mut user_list = String::from("Usuarios registrados: \n");

    for user_data in user_data_vec {
        let username = user_data.username.map_or_else(
            || String::from("sin nombre de usuario"),
            |username| username,
        );

        let full_name = match (user_data.first_name, user_data.last_name) {
            (Some(first_name), Some(last_name)) => format!("{first_name} {last_name}"),
            (Some(first_name), None) => first_name,
            (None, Some(last_name)) => last_name,
            (None, None) => String::new(),
        };

        let user_data = user_data.id;

        user_list += &format!("{username} [<code>{user_data}</code>] {full_name}\n");
    }

    bot.send_message(msg.chat.id, user_list).await?;

    Ok(())
}

///////////////////////////////////////////////////////////\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
//                                       Database section
// \\
//////////////////////////////////////////////////////////\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\

/// # Errors
pub fn read_database_file() -> ResponseResult<String> {
    let contents = fs::read_to_string("database.json")?;

    Ok(contents)
}

/// # Leer la base de datos `database.json`
/// Función para leer la base de datos `database.json` y convertirla en un vector de
/// `UserData`
async fn read_users_from_file() -> Vec<User> {
    let json_str = (read_to_string("database.json").await)
        .map_or_else(|_| String::from("[]"), |json_str| json_str);

    serde_json::from_str(&json_str).map_or_else(|_| vec![], |users| users)
}

fn write_users_to_file(users: &[User]) -> Result<(), io::Error> {
    let json_str = match serde_json::to_string_pretty(&users) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error al convertir a JSON: {e}");

            String::new()
        }
    };

    fs::write("database.json", json_str)
}

fn update_user_data(
    users: &mut Vec<User>,
    user_id: UserId,
    username: String,
    first_name: String,
    last_name: Option<String>,
) {
    for user in users {
        if user.id == user_id {
            user.username = Some(username);

            user.first_name = first_name;

            user.last_name = last_name;

            break;
        }
    }
}

/// # Errors
pub fn get_all_users() -> Result<Vec<UserData>, io::Error> {
    let contents = fs::read_to_string("database.json")?;

    let user_data_vec: Vec<UserData> = serde_json::from_str(&contents)?;

    Ok(user_data_vec)
}

//////////////////////////////////////////////\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
//                                      bot commands
// \\
//////////////////////////////////////////////\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\

/// # Errors
pub async fn get_user_id_by_username(bot: Bot, msg: Message) -> ResponseResult<()> {
    // Obtener el username del mensaje
    let Some(text) = msg.text() else {
        return Ok(())
    };

    let (_, username) = text
        .find('@')
        .map_or(("", text), |index| text.split_at(index));

    let msg_copy = msg.clone();

    let true = username.contains('@') else {
        no_username_found(bot, msg_copy, username).await?;

        return Ok(())
    };

    // Obtener todos los usuarios registrados en el archivo database.json
    let contents = read_database_file()?;

    let user_data_vec: Vec<UserData> = match serde_json::from_str(&contents) {
        Ok(vec) => vec,
        Err(e) => {
            eprintln!("Error parsing user data: {e}");

            Vec::new()
        }
    };

    let user_id = user_data_vec.iter().find_map(|data| match &data.username {
        Some(name) if name == username => Some(data.id.to_string()),
        _ => None,
    });

    // Enviar el user_id como respuesta al usuario
    Box::pin(action_handler(bot, msg_copy, username, user_id)).await?;

    Ok(())
}

/// # Errors
pub async fn action_handler(
    bot: Bot,
    msg: Message,
    username: &str,
    user_id: Option<String>,
) -> ResponseResult<()> {
    match user_id {
        Some(user_id) => {
            let Ok(user_id) = user_id.parse::<u64>() else {
                println!("❌ No se pudo convertir el user_id a u64");

                return Ok(())
            };

            let Some(message) = msg.text() else {
                println!("❌ No se pudo obtener el texto del mensaje {msg:#?}");

                return Ok(()) // Si no hay un mensaje, no se hace nada
            };

            let command = message.split(' ').next().unwrap_or_default();

            match command {
                "/unban" => {
                    unban_for_testing(bot, msg, username, user_id).await?;
                }
                "/ban" => {
                    ban_for_testing(bot, msg, username, UserId(user_id)).await?;
                }
                "/mute" => {
                    mute_for_testing(bot, msg, username, user_id).await?;
                }
                "/unmute" => {
                    unmute_for_testing(bot, msg, username, user_id).await?;
                }
                _ => (),
            }
        }

        None => {
            no_username_found(bot, msg, username).await?;
        }
    };

    Ok(())
}

/// # Errors
async fn unban_for_testing(
    bot: Bot,
    msg: Message,
    username: &str,
    user_id: u64,
) -> ResponseResult<()> {
    // Obtener información sobre el miembro del chat
    let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

    let bot_copy = bot.clone();

    if !chat_member.is_banned() {
        let err = bot
            .send_message(
                msg.chat.id,
                format!("{username} [<code>{user_id}</code>] {NOT_BANNED}"),
            )
            .await?;

        tokio::spawn(async move {
            sleep(Duration::from_secs(60)).await;
            bot.delete_message(msg.chat.id, err.id)
                .await
                .unwrap_or_default();
            bot.delete_message(msg.chat.id, msg.id)
                .await
                .unwrap_or_default();
        });
    }

    bot_copy
        .unban_chat_member(msg.chat.id, UserId(user_id))
        .await?;

    let video = bot_copy
        .send_video(msg.chat.id, InputFile::file("./assets/unban/1.mp4"))
        .caption(format!("{username} [<code>{user_id}</code>] desbaneado."))
        .await?;

    tokio::spawn(async move {
        sleep(Duration::from_secs(60)).await;
        bot_copy
            .delete_message(msg.chat.id, video.id)
            .await
            .unwrap_or_default();
        bot_copy
            .delete_message(msg.chat.id, msg.id)
            .await
            .unwrap_or_default();
    });

    Ok(())
}

/// # Errors
async fn ban_for_testing(
    bot: Bot,
    msg: Message,
    username: &str,
    user_id: UserId,
) -> ResponseResult<()> {
    let chat_member = bot.get_chat_member(msg.chat.id, user_id).await?;

    let bot_copy = bot.clone();

    if chat_member.is_banned() {
        already_banned(bot_copy.clone(), msg.clone(), user_id, username.to_string())
            .await?;
    }

    bot_copy.ban_chat_member(msg.chat.id, user_id).await?;

    ban_animation_generator(bot_copy, msg).await?;

    Ok(())
}

/// # Errors
async fn mute_for_testing(
    bot: Bot,
    msg: Message,
    username: &str,
    user_id: u64,
) -> ResponseResult<()> {
    let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

    let bot_copy = bot.clone();

    if chat_member.is_restricted() {
        let err = bot
            .send_message(
                msg.chat.id,
                format!("{username} [<code>{user_id}</code>] {ALREADY_MUTED}"),
            )
            .await?;

        tokio::spawn(async move {
            sleep(Duration::from_secs(60)).await;
            bot.delete_message(msg.chat.id, err.id)
                .await
                .unwrap_or_default();
            bot.delete_message(msg.chat.id, msg.id)
                .await
                .unwrap_or_default();
        });
    }

    bot_copy
        .restrict_chat_member(msg.chat.id, UserId(user_id), ChatPermissions::empty())
        .await?;

    mute_animation_generator(bot_copy, msg).await?;

    Ok(())
}

/// # Errors
async fn unmute_for_testing(
    bot: Bot,
    msg: Message,
    username: &str,
    user_id: u64,
) -> ResponseResult<()> {
    let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

    let bot_copy = bot.clone();

    if !chat_member.is_restricted() {
        bot.restrict_chat_member(msg.chat.id, UserId(user_id), ChatPermissions::all())
            .await?;

        let ok = bot
            .send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4"))
            .caption(format!("{username} [<code>{user_id}</code>] {UNMUTED}"))
            .await?;

        delete_message_timer(bot_copy.clone(), msg.clone(), ok.id, msg.id, 60);
    }

    bot_copy
        .send_message(
            msg.chat.id,
            format!("{username} [<code>{user_id}</code>] {NOT_MUTED}"),
        )
        .await?;

    Ok(())
}

/// # Errors
pub async fn get_user_id_by_arguments(bot: Bot, msg: Message) -> ResponseResult<()> {
    // extract the text content of the message
    let Some(text) = msg.text() else {
        return Ok(())
    };

    // get the arguments after the command trigger
    let (_, arguments) = text
        .find(' ')
        .map_or(("", text), |index| text.split_at(index));

    // check if the arguments are empty
    if arguments.is_empty() {
        no_arguments(bot, msg).await?;

        return Ok(());
    }

    let true = arguments.contains('@') else {
        let Ok(user_id) = arguments.trim().parse::<u64>() else {
            id_or_username_not_valid(bot, msg).await?;

            return Ok(())
        };

        let Some(from) = msg.from() else {
            println!("❌ No se pudo obtener el usuario que envió el mensaje {msg:#?}");

            return Ok(())
        };

        let is_admin_or_owner = bot
            .get_chat_member(msg.chat.id, from.id)
            .await?
            .is_admin_or_owner();

        if !is_admin_or_owner {
            permissions_denied(bot, msg).await?;

            return Ok(())
        };

        // let false = !is_admin_or_owner else {
        // permissions_denied(bot, msg).await?;
        //
        // return Ok(())
        // };

        let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

        let chat_member_copy = chat_member.clone();

        let first_name = chat_member.user.first_name;

        let username = chat_member.user.username.unwrap_or(first_name);

        if !chat_member_copy.is_banned() {
            ban_for_arguments(bot.clone(), msg.clone(), user_id, username.clone())
                .await?;
        }

        already_banned(bot, msg, UserId(user_id), username).await?;

        return Ok(())
    };

    let Some(from) = msg.from() else {
        return Ok(())
    };

    let is_admin_or_owner = bot
        .get_chat_member(msg.chat.id, from.id)
        .await?
        .is_admin_or_owner();

    if !is_admin_or_owner {
        permissions_denied(bot.clone(), msg.clone()).await?;
    }

    Box::pin(get_user_id_by_username(bot, msg)).await?;

    // let true = is_admin_or_owner else {
    // return Ok(());
    // };

    Ok(())
}

// ////////////||\\\\\\\\\\\\
// //   Ban For Arguments   \\
// \\\\\\\\\\\\||/////////////

/// # Errors
pub async fn ban_for_arguments(
    bot: Bot,
    msg: Message,
    user_id: u64,
    username: String,
) -> ResponseResult<()> {
    let bot_copy = bot.clone();

    bot_copy
        .ban_chat_member(msg.chat.id, UserId(user_id))
        .await?;

    let mut rng: StdRng = SeedableRng::from_entropy();

    let random_number = rng.gen_range(0..=14);

    let file_names = [
        "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif", "8.gif", "9.gif",
        "10.gif", "11.gif", "12.mp4", "13.mp4", "14.mp4",
    ];

    let get_file_name = |index: usize| -> &'static str {
        let Some(file_name) = file_names.get(index) else {
            let Some(last_file) = file_names.last() else {
                println!(
                    "❌ No se pudo obtener el último archivo de la lista de archivos \
                     {file_names:#?}"
                );

                return ""
            };

            return last_file
        };

        file_name
    };

    let file_path = format!("./assets/ban/{}", get_file_name(random_number));

    if !Path::new(&file_path)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("gif"))
    {
        let video = bot_copy
            .send_video(msg.chat.id, InputFile::file(file_path.clone()))
            .caption(format!("@{username} [<code>{user_id}</code>] baneado"))
            .await?;

        delete_message_timer(bot_copy.clone(), msg.clone(), video.id, msg.id, 60);
    }

    let gif = bot
        .send_animation(msg.chat.id, InputFile::file(file_path))
        .caption(format!("@{username} [<code>{user_id}</code>] baneado"))
        .await?;

    delete_message_timer(bot, msg.clone(), gif.id, msg.id, 60);

    Ok(())
}

/// # Errors
pub async fn test_json_two(bot: Bot, msg: Message) -> ResponseResult<()> {
    let Some(user) = msg.from() else {
        println!("No user found.");

        return Ok(())
    };

    let username = user.username.as_ref().map_or_else(
        || user.first_name.clone(),
        |username| format!("@{username}"),
    );

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

    if is_registered {
        update_user_data(
            &mut users,
            user_id,
            username.clone(),
            first_name.clone(),
            last_name.clone(),
        );
    } else {
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
    }

    let result = write_users_to_file(&users);

    match result {
        Ok(()) => println!("User saved to database.json"),
        Err(e) => println!("Error writing to database: {e:?}"),
    }

    if username != db_username.clone().unwrap_or_default() {
        send_username_changed_message(
            bot.clone(),
            msg.chat.id,
            &user.first_name,
            db_username,
            &username,
        )
        .await?;
    }

    if first_name != db_first_name.clone().unwrap_or_default() {
        send_first_name_changed_message(
            bot.clone(),
            msg.chat.id,
            &user.first_name,
            db_first_name,
            &first_name,
        )
        .await?;
    }

    if last_name != db_last_name.clone().unwrap_or_default() {
        send_last_name_changed_message(
            bot,
            msg.chat.id,
            &user.first_name,
            db_last_name.unwrap_or_default(),
            last_name,
        )
        .await?;
    }

    Ok(())
}

async fn send_username_changed_message(
    bot: Bot,
    chat_id: ChatId,
    username: &String,
    old_username: Option<String>,
    new_username: &str,
) -> ResponseResult<()> {
    let old_username = old_username.unwrap_or_default();

    bot.send_message(
        chat_id,
        format!(
            "{username} cambió su nombre de usuario de {old_username} a {new_username}."
        ),
    )
    .await?;

    Ok(())
}

async fn send_first_name_changed_message(
    bot: Bot,
    chat_id: ChatId,
    username: &str,
    old_first_name: Option<String>,
    new_first_name: &str,
) -> ResponseResult<()> {
    bot.send_message(
        chat_id,
        format!("{username} cambió su nombre de {old_first_name:?} a {new_first_name}."),
    )
    .await?;

    Ok(())
}

async fn send_last_name_changed_message(
    bot: Bot,
    chat_id: ChatId,
    username: &str,
    old_last_name: Option<String>,
    new_last_name: Option<String>,
) -> ResponseResult<()> {
    let old_last_name_text =
        old_last_name.map_or_else(|| String::from("ninguno"), |last_name| last_name);

    let new_last_name_text =
        new_last_name.map_or_else(|| String::from("ninguno"), |last_name| last_name);

    bot.send_message(
        chat_id,
        format!("{username} cambió su apellido de {old_last_name_text} a {new_last_name_text}."),
    )
    .await?;

    Ok(())
}
