use crate::admin_commands::*;

pub async fn get_chat_member(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = replied.from().unwrap();
            let id_usuario = user.id;
            let username_user = user.username.clone().unwrap_or_default();
            let first_name = &user.first_name;
            bot.send_message(msg.chat.id, format!("<b>Nombre:</b> {} \n<b>Username:</b> @{:} \n<b>ID: </b><code>{}</code>", first_name, username_user, id_usuario)).parse_mode(ParseMode::Html).await?;
        }

        None => {
            bot.send_message(msg.chat.id, "❌ No se ha respondido a ningún mensaje").await?;
        }
    }

    Ok(())
}