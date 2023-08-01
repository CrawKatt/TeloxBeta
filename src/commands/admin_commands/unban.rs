use crate::dependencies::*;

/// # Errors
/// # Panics
pub async fn unban_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let Some(user) = replied.from() else {
                let error_msg = bot.send_message(msg.chat.id, "❌ No se pudo obtener el usuario").await?;
                let error_msg_id = error_msg.id;

                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, error_msg_id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;

                return Ok(());
            };

            if let Some(from) = msg.from() {
                let username_user = user.clone().username.map_or_else(String::new, |username| username);

                let is_admin_or_owner = bot.get_chat_member(msg.chat.id, from.id).await?.is_admin_or_owner();
                if is_admin_or_owner {
                    let chat_member = bot.get_chat_member(msg.chat.id, user.id).await?;
                    if chat_member.status().is_banned() {
                        bot.unban_chat_member(msg.chat.id, user.id).await?;

                        let ok = bot
                            .send_video(msg.chat.id, InputFile::file("./assets/unban/1.mp4"))
                            .caption(format!("✅ @{username_user} desbaneado",))
                            .reply_to_message_id(msg.id)
                            .await?;

                        sleep(Duration::from_secs(60)).await;
                        bot.delete_message(msg.chat.id, ok.id).await?;
                    } else {
                        let err = bot
                            .send_message(msg.chat.id, format!("❌ @{username_user} {NOT_BANNED}"))
                            .reply_to_message_id(msg.id)
                            .await?;

                        sleep(Duration::from_secs(10)).await;
                        bot.delete_message(msg.chat.id, err.id).await?;
                        bot.delete_message(msg.chat.id, msg.id).await?;
                        return Ok(());
                    }
                } else {
                    let err =
                        bot.send_message(msg.chat.id, format!("❌ {PERMISSIONS_DENIED}")).await?;
                    sleep(Duration::from_secs(5)).await;
                    bot.delete_message(msg.chat.id, err.id).await?;
                }
            }
        }

        None => {
            Box::pin(get_user_id_by_arguments_for_unban(bot, msg)).await?;
        }
    }

    Ok(())
}

/// # Errors
/// # Panics
pub async fn get_user_id_by_arguments_for_unban(bot: Bot, msg: Message) -> ResponseResult<()> {
    // extract the text content of the message

    let Some(text) = msg.text() else {
        return Ok(());
    };

    // get the arguments after the command trigger
    let (_, arguments) = text.find(' ').map_or(("", text), |index| text.split_at(index));

    // check if the arguments are empty
    if arguments.is_empty() {
        bot.send_message(msg.chat.id, "❌ No has especificado un ID para obtener el usuario")
            .await?;
        bot.delete_message(msg.chat.id, msg.id).await?;
        println!("❌ No has especificado un ID para obtener el usuario {msg:#?}");

        return Ok(());
    }

    // if arguments is String, then use this
    if arguments.contains('@') {
        let Some(from) = msg.from()  else {
            return Ok(());
        };

        let is_admin_or_owner =
            bot.get_chat_member(msg.chat.id, from.id).await?.is_admin_or_owner();
        let true = is_admin_or_owner else {
            bot.send_message(msg.chat.id, "❌ No tienes permisos para usar este comando").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
            println!("❌ No tienes permisos para usar este comando {msg:#?}");
            return Ok(());
        };
        Box::pin(get_user_id_by_username(bot, msg)).await?;
    } else {
        // extract the user ID from the arguments
        let Ok(user_id) = arguments.trim().parse::<u64>() else {
            let err = bot.send_message(msg.chat.id, "❌ El ID o @Username proporcionado no es válido, considera reenviar un mensaje al bot para hacer un ban por ID").await?;
            sleep(Duration::from_secs(5)).await;
            bot.delete_message(msg.chat.id, err.id).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;

            return Ok(());
        };

        let Some(from) = msg.from() else {
            println!("❌ No se pudo obtener el usuario que envió el mensaje {msg:#?}");
            return Ok(());
        };

        //let chat_member = bot.get_chat_member(msg.chat.id, from.id).await?;
        // check if the user is an admin or owner of the chat
        //let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;
        // If the user is an admin or owner, ban the target user and send a ban message.
        let is_admin_or_owner =
            bot.get_chat_member(msg.chat.id, from.id).await?.is_admin_or_owner();

        let false = !is_admin_or_owner else {
            let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para usar este comando").await?;
            sleep(Duration::from_secs(5)).await;
            bot.delete_message(msg.chat.id, err.id).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
            return Ok(());
        };

        let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;
        let username = chat_member.user.username.clone().unwrap_or_else(|| "no username".to_string());

        let ChatMemberStatus::Banned { .. } = chat_member.status() else {
            bot.send_message(msg.chat.id, format!("❌ @{username} [<code>{user_id}</code>] No está Baneado"))
                .await?;

            sleep(Duration::from_secs(5)).await;
            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
            ban_animation_generator(bot, msg).await?;
            return Ok(());
        };

        bot.unban_chat_member(msg.chat.id, UserId(user_id)).await?;
        let mute_ok = bot
            .send_message(
                msg.chat.id,
                format!("✅ @{username} [<code>{user_id}</code>] Ya no está Baneado"),
            )
            .await?;

        sleep(Duration::from_secs(5)).await;
        bot.delete_message(msg.chat.id, mute_ok.id).await?;
        bot.delete_message(msg.chat.id, msg.id).await?;
        return Ok(());
    }

    Ok(())
}
