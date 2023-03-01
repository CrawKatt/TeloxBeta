use crate::commands::*;

pub async fn anti_spam(bot: Bot, msg: Message) -> ResponseResult<()> {

    if let Some(from) = msg.from() {
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.send_message(msg.chat.id, "Enlace Spam Detectado\\!\nAcción: Baneado").await?;
        bot.ban_chat_member(msg.chat.id, from.id).await?;
        ban_animation_generator(bot, msg).await?;
    } else {
        println!("No se pudo obtener el usuario");
    }

    Ok(())
}