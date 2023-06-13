use crate::admin_commands::*;

pub async fn mute_user_admin(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = if let Some(from) = replied.from() {
                from
            } else {
                // Send an error message and delete it after 5 seconds.
                let error_msg = bot.send_message(msg.chat.id, "❌ No se pudo obtener el usuario")
                    .reply_to_message_id
                    (msg.id)
                    .await?;

                let error_msg_id = error_msg.id;

                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, error_msg_id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;

                return Ok(());
            };

            if let Some(from) = msg.from() {

                let username_user = match user.clone().username {
                    Some(username) => username,
                    None => String::new(),
                };

                let is_admin_or_owner = bot.get_chat_member(msg.chat.id, from.id).await?.is_admin_or_owner();
                if is_admin_or_owner {

                    let chat_member = bot.get_chat_member(msg.chat.id, user.id).await?;
                    if chat_member.status().is_restricted() {
                        let err = bot.send_message(msg.chat.id, format!
                        ("❌ @{} Ya está silenciado. Usa este comando solo para silenciar a alguien que no esté silenciado", username_user))
                            .reply_to_message_id(msg.id)
                            .parse_mode(ParseMode::Html)
                            .await?;

                        sleep(Duration::from_secs(10)).await;
                        bot.delete_message(msg.chat.id, err.id).await?;
                        bot.delete_message(msg.chat.id, msg.id).await?;
                        return Ok(());

                    } else {
                        bot.restrict_chat_member(msg.chat.id, user.id, ChatPermissions::empty()).await?;

                        let mut rng: StdRng = SeedableRng::from_entropy();
                        let random_number = rng.gen_range(0..=14);
                        let file_names = [
                            "1.gif", "2.gif", "3.gif", "4.gif", "5.jpg",
                        ];

                        let get_file_name = |index: usize| -> &'static str {
                            file_names.get(index).unwrap_or_else(|| file_names.last().unwrap())
                        };
                        let file_path = format!("./assets/mute/{}", get_file_name(random_number));

                        match file_path.ends_with(".gif") {

                            true => {
                                bot.send_animation(msg.chat.id, InputFile::file(file_path))
                                    .caption(format!("✅ @{} [<code>{}</code>] silenciado", username_user, user.id))
                                    .parse_mode(ParseMode::Html)
                                    .reply_to_message_id(msg.id)
                                    .await?;
                            }

                            false => {
                                bot.send_photo(msg.chat.id, InputFile::file(file_path))
                                    .caption(format!("✅ @{} [<code>{}</code>] silenciado", username_user, user.id))
                                    .parse_mode(ParseMode::Html)
                                    .reply_to_message_id(msg.id)
                                    .await?;
                            }

                        }

                    }

                } else {
                    let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para silenciar a un usuario").await?;
                    sleep(Duration::from_secs(5)).await;
                    bot.delete_message(msg.chat.id, err.id).await?;
                    bot.delete_message(msg.chat.id, msg.id).await?;
                };

            }

        }

        None => {
            get_user_id_by_arguments_for_mute(bot, msg).await?;
        }

    }

    Ok(())
}

pub async fn get_user_id_by_arguments_for_mute(bot: Bot, msg: Message) -> ResponseResult<()> {
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
        println!("❌ No has especificado un ID para obtener el usuario {msg:#?}");

        return Ok(());
    }

    // if arguments is String, then use this
    if arguments.contains('@') {

        let Some(from) = msg.from()  else {
            return Ok(());
        };

        let is_admin_or_owner = bot.get_chat_member(msg.chat.id, from.id).await?.is_admin_or_owner();
        let true = is_admin_or_owner else {
            bot.send_message(msg.chat.id, "❌ No tienes permisos para usar este comando").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
            println!("❌ No tienes permisos para usar este comando {msg:#?}");
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
            println!("❌ No se pudo obtener el usuario que envió el mensaje {msg:#?}");
            return Ok(());
        };

        // check if the user is an admin or owner of the chat
        let is_admin_or_owner = bot.get_chat_member(msg.chat.id, from.id).await?.is_admin_or_owner();
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
            bot.restrict_chat_member(msg.chat.id, UserId(user_id), ChatPermissions::empty()).await?;
            let mute_ok = bot.send_message(msg.chat.id, format!("✅ @{username} [<code>{user_id}</code>] Silenciado"))
                .parse_mode(ParseMode::Html)
                .await?;

            sleep(Duration::from_secs(5)).await;
            bot.delete_message(msg.chat.id, mute_ok.id).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
            ban_animation_generator(bot, msg).await?;
            return Ok(());
        };

        let err = bot.send_message(msg.chat.id, format!("❌ @{username} [<code>{user_id}</code>] Ya está silenciado"))
            .parse_mode(ParseMode::Html)
            .await?;

        sleep(Duration::from_secs(5)).await;
        bot.delete_message(msg.chat.id, err.id).await?;
        bot.delete_message(msg.chat.id, msg.id).await?;
        return Ok(());
    }

    Ok(())
}
