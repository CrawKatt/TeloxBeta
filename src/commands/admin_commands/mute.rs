use crate::dependencies::*;

/// # Errors
/// # Panics
pub async fn mute_user_admin(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let Some(user) = replied.from() else {
                // Send an error message and delete it after 5 seconds.
                let error_msg = bot
                    .send_message(msg.chat.id, "❌ No se pudo obtener el usuario")
                    .reply_to_message_id(msg.id)
                    .await?;

                let err = error_msg.id;

                tokio::spawn(async move {
                    sleep(Duration::from_secs(5)).await;
                    bot.delete_message(msg.chat.id, err)
                        .await
                        .unwrap_or_default();
                    bot.delete_message(msg.chat.id, msg.id)
                        .await
                        .unwrap_or_default();
                });

                return Ok(())
            };

            if let Some(from) = msg.from() {
                let username_user = user
                    .clone()
                    .username
                    .map_or_else(String::new, |username| username);

                let is_admin_or_owner = bot
                    .get_chat_member(msg.chat.id, from.id)
                    .await?
                    .is_admin_or_owner();

                if is_admin_or_owner {
                    let chat_member = bot.get_chat_member(msg.chat.id, user.id).await?;

                    if chat_member.status().is_restricted() {
                        let err = bot
                            .send_message(
                                msg.chat.id,
                                format!("❌ @{username_user} {ALREADY_MUTED}"),
                            )
                            .reply_to_message_id(msg.id)
                            .await?;

                        tokio::spawn(async move {
                            sleep(Duration::from_secs(10)).await;
                            bot.delete_message(msg.chat.id, err.id)
                                .await
                                .unwrap_or_default();
                            bot.delete_message(msg.chat.id, msg.id)
                                .await
                                .unwrap_or_default();
                        });

                        return Ok(())
                    }

                    bot.restrict_chat_member(
                        msg.chat.id,
                        user.id,
                        ChatPermissions::empty(),
                    )
                    .await?;

                    let mut rng: StdRng = SeedableRng::from_entropy();

                    let random_number = rng.gen_range(0..=14);

                    let file_names = ["1.gif", "2.gif", "3.gif", "4.gif", "5.jpg"];

                    let get_file_name = |index: usize| -> &'static str {
                        file_names
                            .get(index)
                            .unwrap_or_else(|| file_names.last().unwrap())
                    };

                    let file_path =
                        format!("./assets/mute/{}", get_file_name(random_number));

                    let user_id = user.id;
                    if Path::new(&file_path)
                        .extension()
                        .map_or(false, |ext| ext.eq_ignore_ascii_case("gif"))
                    {
                        bot.send_animation(msg.chat.id, InputFile::file(file_path))
                            .caption(format!(
                                "✅ @{username_user} [<code>{user_id}</code>] silenciado",
                            ))
                            .reply_to_message_id(msg.id)
                            .await?;
                    } else {
                        let user_id = user.id;

                        bot.send_photo(msg.chat.id, InputFile::file(file_path))
                            .caption(format!(
                                "✅ @{username_user} [<code>{user_id}</code>] silenciado",
                            ))
                            .reply_to_message_id(msg.id)
                            .await?;
                    }
                } else {
                    let err = bot
                        .send_message(
                            msg.chat.id,
                            "❌ No tienes permisos para silenciar a un usuario",
                        )
                        .await?;

                    tokio::spawn(async move {
                        sleep(Duration::from_secs(10)).await;
                        bot.delete_message(msg.chat.id, err.id)
                            .await
                            .unwrap_or_default();
                        bot.delete_message(msg.chat.id, msg.id)
                            .await
                            .unwrap_or_default();
                    });
                };
            }
        }

        None => {
            Box::pin(get_user_id_by_arguments_for_mute(bot, msg)).await?;
        }
    }

    Ok(())
}

/// # Errors
pub async fn get_user_id_by_arguments_for_mute(
    bot: Bot,
    msg: Message,
) -> ResponseResult<()> {
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
        let err = bot
            .send_message(
                msg.chat.id,
                "❌ No has especificado un ID para obtener el usuario",
            )
            .await?;

        tokio::spawn(async move {
            sleep(Duration::from_secs(5)).await;
            bot.delete_message(msg.chat.id, err.id)
                .await
                .unwrap_or_default();
            bot.delete_message(msg.chat.id, msg.id)
                .await
                .unwrap_or_default();
        });

        return Ok(())
    }

    // if arguments is String, then use this
    if arguments.contains('@') {
        let Some(from) = msg.from() else {
            return Ok(())
        };

        let is_admin_or_owner = bot
            .get_chat_member(msg.chat.id, from.id)
            .await?
            .is_admin_or_owner();

        let true = is_admin_or_owner else {
            let err = bot
                .send_message(msg.chat.id, "❌ No tienes permisos para usar este comando")
                .await?;

            tokio::spawn(async move {
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, err.id)
                    .await
                    .unwrap_or_default();
                bot.delete_message(msg.chat.id, msg.id)
                    .await
                    .unwrap_or_default();
            });

            return Ok(())
        };

        Box::pin(get_user_id_by_username(bot, msg)).await?;
    } else {
        // extract the user ID from the arguments
        let Ok(user_id) = arguments.trim().parse::<u64>() else {
            let err = bot
                .send_message(
                    msg.chat.id,
                    "❌ El ID o @Username proporcionado no es válido, considera reenviar un \
                     mensaje al bot para hacer un ban por ID",
                )
                .await?;

            tokio::spawn(async move {
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, err.id)
                    .await
                    .unwrap_or_default();
                bot.delete_message(msg.chat.id, msg.id)
                    .await
                    .unwrap_or_default();
            });

            return Ok(())
        };

        let Some(from) = msg.from() else {
            println!("❌ No se pudo obtener el usuario que envió el mensaje {msg:#?}");

            return Ok(())
        };

        // check if the user is an admin or owner of the chat
        let is_admin_or_owner = bot
            .get_chat_member(msg.chat.id, from.id)
            .await?
            .is_admin_or_owner();

        // If the user is an admin or owner, ban the target user and send a ban message.

        let false = !is_admin_or_owner else {
            let err = bot
                .send_message(msg.chat.id, "❌ No tienes permisos para usar este comando")
                .await?;

            tokio::spawn(async move {
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, err.id)
                    .await
                    .unwrap_or_default();
                bot.delete_message(msg.chat.id, msg.id)
                    .await
                    .unwrap_or_default();
            });

            return Ok(())
        };

        let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

        let username = chat_member
            .user
            .username
            .clone()
            .unwrap_or_else(|| "no username".to_string());

        let ChatMemberStatus::Banned { .. } = chat_member.status() else {
            let bot_copy = bot.clone();

            bot.restrict_chat_member(
                msg.chat.id,
                UserId(user_id),
                ChatPermissions::empty(),
            )
            .await?;

            let ok = bot
                .send_message(
                    msg.chat.id,
                    format!("✅ @{username} [<code>{user_id}</code>] Silenciado"),
                )
                .await?;

            tokio::spawn(async move {
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, ok.id)
                    .await
                    .unwrap_or_default();
                bot.delete_message(msg.chat.id, msg.id)
                    .await
                    .unwrap_or_default();
            });

            ban_animation_generator(bot_copy, msg).await?;

            return Ok(())
        };

        let err = bot
            .send_message(
                msg.chat.id,
                format!("❌ @{username} [<code>{user_id}</code>] Ya está silenciado"),
            )
            .await?;

        tokio::spawn(async move {
            sleep(Duration::from_secs(5)).await;
            bot.delete_message(msg.chat.id, err.id)
                .await
                .unwrap_or_default();
            bot.delete_message(msg.chat.id, msg.id)
                .await
                .unwrap_or_default();
        });

        return Ok(())
    }

    Ok(())
}
