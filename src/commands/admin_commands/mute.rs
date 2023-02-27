use crate::admin_commands::*;

pub async fn mute_user_admin(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = replied.from().unwrap();
            let chat_id = msg.chat.id;
            let username_user = user.username.clone().unwrap_or_default();
            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
            let is_admin_or_owner = chat_member.status().is_administrator() || chat_member.status().is_owner();

            if is_admin_or_owner {
                bot.restrict_chat_member(chat_id, user.id, ChatPermissions::empty()).await?;
                bot.send_message(chat_id, format!("✅ @{} ha sido silenciado", username_user)).await?;
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(chat_id, msg.id).await?;
                mute_animation_generator(bot, msg).await?;

            } else {
                let err = bot.send_message(chat_id, "❌ No tienes permisos para silenciar a un usuario").await?;
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(chat_id, err.id).await?;
                bot.delete_message(chat_id, msg.id).await?;
            };

        }
        None => {
            let text = &msg.text().unwrap();
            let (_, arguments) = text.split_at(text.find(' ').unwrap_or(text.len()));
            let user_id = arguments.trim().parse::<i64>().unwrap();
            let chat_id = msg.chat.id;
            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
            let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;

            if is_admin_or_owner {
                bot.restrict_chat_member(chat_id, UserId(user_id as u64), ChatPermissions::empty()).await?;
                let ok_mute = bot.send_message(msg.chat.id, format!("Silenciado \\[`{}`\\]", user_id)).await?;
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, ok_mute.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
                mute_animation_generator(bot, msg).await?;

            } else {
                bot.send_message(msg.chat.id, "❌ No tienes permisos para silenciar a un usuario", ).await?;
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, msg.id).await?;
            }
        }
    }

    Ok(())
}