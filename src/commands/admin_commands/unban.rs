use crate::admin_commands::*;

pub async fn unban_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = if let Some(from) = replied.from() {
                from
            } else {
                let error_msg = bot.send_message(msg.chat.id, "❌ No se pudo obtener el usuario").await?;
                let error_msg_id = error_msg.id;

                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, error_msg_id).await?;

                sleep(Duration::from_secs(1)).await;
                bot.delete_message(msg.chat.id, msg.id).await?;
                return Ok(());
            };

            let chat_id = msg.chat.id;
            let username_user = user.username.clone().unwrap_or_default();
            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
            let is_admin_or_owner = chat_member.status().is_administrator() || chat_member.status().is_owner();

            if is_admin_or_owner {
                bot.unban_chat_member(chat_id, user.id).await?;
                let ok_unban = bot.send_message(chat_id, format!("✅ @{} desbaneado", username_user)).await?;
                bot.send_video(chat_id, InputFile::file("./assets/unban/1.mp4")).await?;
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(chat_id, ok_unban.id).await?;
                bot.delete_message(chat_id, msg.id).await?;
            } else {
                let err = bot.send_message(chat_id, "❌ No tienes permisos para remover el ban a un usuario.").await?;
                bot.delete_message(chat_id, err.id).await?;
            }

        }

        None => {
            let text = &msg.text().unwrap();
            let (_, arguments) = text.split_at(text.find(' ').unwrap_or(text.len()));
            let user_id = arguments.trim().parse::<i64>().unwrap();
            let chat_id = msg.chat.id;
            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
            let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;

            if is_admin_or_owner {
                bot.unban_chat_member(chat_id, UserId(user_id as u64)).await?;
                let ok_unban = bot.send_message(msg.chat.id, format!("✅ Desbaneado \\[`{}`\\]", user_id)).await?;
                bot.send_video(chat_id, InputFile::file("./assets/unban/1.mp4")).await?;

                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, ok_unban.id).await?;
            } else {
                let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para remover el ban a un usuario", ).await?;
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, err.id).await?;
            }

        }
    }

    Ok(())
}