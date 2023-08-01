// use crate::admin_commands::*;
//
// pub static mut ANTI_SPAM_ENABLED: bool = true;
//
// pub async fn anti_spam(bot: Bot, msg: Message) -> ResponseResult<()> {
//
// if !unsafe { ANTI_SPAM_ENABLED } {
// return Ok(());
// }
//
// if let Some(from) = msg.from() {
// let chat_member = bot.get_chat_member(msg.chat.id, from.id).await?;
// let is_admin_or_owner = chat_member.status() ==
// ChatMemberStatus::Administrator || chat_member.status() ==
// ChatMemberStatus::Owner;
//
// if is_admin_or_owner {
// println!("El usuario es administrador");
// return Ok(())
// } else {
// bot.delete_message(msg.chat.id, msg.id).await?;
// bot.send_message(msg.chat.id, "Enlace Spam Detectado\\!\nAcción:
// Baneado").await?; bot.ban_chat_member(msg.chat.id, from.id).await?;
// ban_animation_generator(bot, msg).await?;
// }
//
// } else {
// println!("No se pudo obtener el usuario");
// }
//
// Ok(())
// }
