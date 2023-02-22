use crate::admin_commands::*;

pub async fn unmute_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = replied.from().unwrap();
            println!("Info : {:?}", user);

            let chat = msg.chat.id;
            println!("Chat ID : {:?}", chat);

            let usuario = user.clone().first_name;
            println!("Nombre : {:?}", usuario);

            let username_user = user.clone().username;
            println!("@username : {:?}", username_user);

            let chat_member = bot
                .get_chat_member(msg.chat.id, msg.from().unwrap().id)
                .await?;

            let user_id = chat_member.user.id;
            println!("ID usuario : {:?}", user_id);

            let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator
                || chat_member.status() == ChatMemberStatus::Owner;
            println!("Es admin o owner: {} \n", is_admin_or_owner);

            if is_admin_or_owner {
                bot.delete_message(msg.chat.id, msg.id).await?;
                bot.restrict_chat_member(msg.chat.id, user.id, ChatPermissions::all())
                    .await?;
                bot.send_message(
                    msg.chat.id,
                    format!("✅ @{} ya no está silenciado", username_user.unwrap()),
                )
                    .await?;
                bot.send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4"))
                    .await?;
            } else {
                bot.delete_message(msg.chat.id, msg.id).await?;
                bot.send_message(
                    msg.chat.id,
                    "❌ No tienes permisos para remover el silencio a un usuario",
                )
                    .await?;
            }
        }
        None => {
            let text = &msg.text().unwrap();
            let (_, arguments) = text.split_at(text.find(' ').unwrap_or(text.len()));
            let user_id = arguments.trim().parse::<i64>().unwrap();

            let chat_id = msg.chat.id;

            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;

            let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator
                || chat_member.status() == ChatMemberStatus::Owner;

            if is_admin_or_owner {
                bot.restrict_chat_member(chat_id, UserId(user_id as u64), ChatPermissions::all())
                    .await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
                bot.send_message(
                    msg.chat.id,
                    format!("\\[`{}`\\] ya no esta silenciado", user_id),
                )
                    .await?;
                bot.send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4"))
                    .await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    "❌ No tienes permisos para remover el silencio a un usuario",
                )
                    .await?;
            }
        }
    }

    Ok(())
}