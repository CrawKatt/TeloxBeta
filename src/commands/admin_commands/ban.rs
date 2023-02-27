use crate::commands::*;
use crate::utils::ban_animation_generator;

pub async fn ban_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    // The function takes a bot and a message object, and returns a Result.
    match msg.reply_to_message() {
        // Check if the message is a reply to another message.
        Some(replied) => {
            // If it is, extract the user from the replied message.
            // If the user cannot be extracted, send an error message.
            let user = if let Some(from) = replied.from() {
                from
            } else {
                // Send an error message and delete it after 5 seconds.
                let error_msg = bot.send_message(msg.chat.id, "❌ No se pudo obtener el usuario").reply_to_message_id(msg.id).await?;
                let error_msg_id = error_msg.id;

                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, error_msg_id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;

                return Ok(());
            };

            // Get the chat ID and user ID, and check if the person using the command is an admin or owner.
            let chat_id = msg.chat.id;
            let username_user = user.username.clone().unwrap_or_default();
            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;

            // If the user is an admin or owner, ban the user and send a message to the chat.
            // Also send a random GIF or MP4 file from the "./assets/ban/" folder.
            let is_admin_or_owner = chat_member.status().is_administrator() || chat_member.status().is_owner();

            if is_admin_or_owner {
                bot.ban_chat_member(chat_id, user.id).await?;
                let ban_msg = bot.send_message(chat_id, format!("✅ @{} \\[`{}`\\] baneado", username_user, user.id)).reply_to_message_id(msg.id).await?;

                sleep(Duration::from_secs(5)).await;
                bot.delete_message(chat_id, ban_msg.id).await?;
                bot.delete_message(chat_id, msg.id).await?;

                // Choose a random ban animation to send.
                ban_animation_generator(bot, msg).await?;
            } else {
                // If the user is not an admin or owner, send an error message and delete this message in 5 seconds.
                let err = bot.send_message(chat_id, "❌ No tienes permisos para usar este comando").reply_to_message_id(msg.id).await?;

                sleep(Duration::from_secs(5)).await;
                bot.delete_message(chat_id, err.id).await?;
                bot.delete_message(chat_id, msg.id).await?;
            };

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
