use crate::dependencies::*;

pub async fn ban_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    // The function takes a bot and a message object, and returns a Result.
    match msg.reply_to_message() {
        // Check if the message is a reply to another message.
        Some(replied) => {
            // If it is, extract the user from the replied message.
            // If the user cannot be extracted, send an error message.
            /*
            let user = if let Some(from) = replied.from() {
                from
            } else {
                // Send an error message and delete it after 5 seconds.
                let err = bot
                    .send_message(msg.chat.id, "❌ No se pudo obtener el usuario")
                    .reply_to_message_id(msg.id)
                    .await?;

                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, err.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;

                return Ok(());
            };
            */
            let Some(user) = replied.from() else {
                let err = bot
                    .send_message(msg.chat.id, "❌ No se pudo obtener el usuario")
                    .reply_to_message_id(msg.id)
                    .await?;

                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, err.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;

                return Ok(())
            };

            // Get the chat ID and user ID, and check if the person using the command is an admin or owner.
            if let Some(from) = msg.from() {
                let username_user =
                    user.clone().username.map_or_else(|| String::new(), |username| username);

                // If the user is an admin or owner, ban the user and send a message to the chat.
                // Also send a random GIF or MP4 file from the "./assets/ban/" folder.
                let is_admin_or_owner =
                    bot.get_chat_member(msg.chat.id, from.id).await?.is_admin_or_owner();

                if is_admin_or_owner {
                    let chat_member = bot.get_chat_member(msg.chat.id, user.id).await?;
                    if chat_member.status().is_banned() {
                        let err = bot
                            .send_message(
                                msg.chat.id,
                                format!("❌ {username_user} {ALREADY_BANNED}"),
                            )
                            .reply_to_message_id(msg.id)
                            .parse_mode(ParseMode::Html)
                            .await?;

                        sleep(Duration::from_secs(5)).await;
                        bot.delete_message(msg.chat.id, err.id).await?;
                        bot.delete_message(msg.chat.id, msg.id).await?;

                        return Ok(());
                    } else {
                        bot.ban_chat_member(msg.chat.id, user.id).await?;
                        let mut rng: StdRng = SeedableRng::from_entropy();
                        let random_number = rng.gen_range(0..=14);
                        let file_names = [
                            "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif", "8.gif",
                            "9.gif", "10.gif", "11.gif", "12.mp4", "13.mp4", "14.mp4",
                        ];
                        let get_file_name = |index: usize| -> &'static str {
                            file_names.get(index).unwrap_or_else(|| file_names.last().unwrap())
                        };
                        let file_path = format!("./assets/ban/{}", get_file_name(random_number));

                        match file_path.ends_with(".gif") {
                            true => {
                                bot.send_animation(msg.chat.id, InputFile::file(file_path))
                                    .caption(format!(
                                        "✅ @{username_user} [<code>{}</code>] baneado",
                                        user.id
                                    ))
                                    .parse_mode(ParseMode::Html)
                                    .reply_to_message_id(msg.id)
                                    .await?;
                            }

                            false => {
                                bot.send_video(msg.chat.id, InputFile::file(file_path))
                                    .caption(format!(
                                        "✅ @{username_user} <code>[{}</code>] baneado",
                                        user.id
                                    ))
                                    .parse_mode(ParseMode::Html)
                                    .reply_to_message_id(msg.id)
                                    .await?;
                            }
                        }
                    }
                } else {
                    // If the user is not an admin or owner, send an error message and delete this message in 5 seconds.
                    let err = bot
                        .send_message(msg.chat.id, "❌ No tienes permisos para usar este comando")
                        .reply_to_message_id(msg.id)
                        .await?;

                    sleep(Duration::from_secs(5)).await;
                    bot.delete_message(msg.chat.id, err.id).await?;
                    bot.delete_message(msg.chat.id, msg.id).await?;
                };
            }
        }
        // If the message is not a reply, extract the user ID from the command's arguments.
        // Check if the person using the command is an admin or owner.
        None => {
            get_user_id_by_arguments(bot, msg).await?;
        }
    }

    Ok(())
}

/*
╔═════════════════════════════════════════════════════╗
║    || - || Desarrollado por @CrawKatt || - ||       ║
║    --| https://github.com/CrawKatt/TeloxBeta |--    ║
╚═════════════════════════════════════════════════════╝
*/
