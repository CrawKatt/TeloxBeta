use crate::admin_commands::*;

pub async fn unmute_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = replied.from().unwrap();
            let username_user = user.clone().username;
            let chat_member = bot.get_chat_member(msg.chat.id, msg.from().unwrap().id).await?;

            let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;
            println!("Es admin o owner: {:#?} \n", is_admin_or_owner);

            if is_admin_or_owner {
                bot.restrict_chat_member(msg.chat.id, user.id, ChatPermissions::all()).await?;
                let ok_unmute = bot.send_message(msg.chat.id, format!("✅ @{} ya no está silenciado", username_user.unwrap()),).await?;
                bot.send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4")).await?;
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, ok_unmute.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
            } else {
                let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para remover el silencio a un usuario", ).await?;
                bot.delete_message(msg.chat.id, err.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
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
                bot.restrict_chat_member(chat_id, UserId(user_id as u64), ChatPermissions::all()).await?;
                let ok_unmute = bot.send_message(msg.chat.id, format!("\\[`{}`\\] ya no esta silenciado", user_id)).await?;
                bot.send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4")).await?;
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, ok_unmute.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
            } else {
                let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para remover el silencio a un usuario").await?;
                bot.delete_message(msg.chat.id, err.id).await?;
            }

        }
    }

    Ok(())
}