use crate::admin_commands::*;

pub async fn unmute_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = if let Some(from) = replied.from() {
                from
            } else {
                let error_msg = bot.send_message(msg.chat.id, "❌ No se pudo obtener el usuario").await?;
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

                let chat_member = bot.get_chat_member(msg.chat.id, from.id).await?;
                let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;

                if is_admin_or_owner {

                    let chat_member = bot.get_chat_member(msg.chat.id, user.id).await?.can_send_messages();
                        if !chat_member {
                            bot.restrict_chat_member(msg.chat.id, user.id, ChatPermissions::all()).await?;
                            let ok_unmute = bot.send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4"))
                                .caption(format!("✅ @{} Ya no está silenciado", username_user))
                                .parse_mode(ParseMode::Html)
                                .reply_to_message_id(msg.id)
                                .await?;

                            sleep(Duration::from_secs(5)).await;
                            bot.delete_message(msg.chat.id, ok_unmute.id).await?;
                            bot.delete_message(msg.chat.id, msg.id).await?;
                        } else {
                            let err = bot.send_message(msg.chat.id, format!(
                                "❌ @{} No está silenciado. Usa este comando solo para remover el silencia de alguien que esté silenciado",
                                username_user))
                                .parse_mode(ParseMode::Html)
                                .reply_to_message_id(msg.id)
                                .await?;

                            sleep(Duration::from_secs(10)).await;
                            bot.delete_message(msg.chat.id, err.id).await?;
                            bot.delete_message(msg.chat.id, msg.id).await?;
                            return Ok(());
                        }

                } else {

                    let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para remover el silencio a un usuario", ).await?;
                    bot.delete_message(msg.chat.id, err.id).await?;
                    bot.delete_message(msg.chat.id, msg.id).await?;
                }

            }

        }

        None => {
            get_user_id_by_arguments(bot, msg).await?;
        }

    }

    Ok(())
}