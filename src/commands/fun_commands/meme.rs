use crate::commands::admin_commands::*;

pub async fn send_random_meme(bot: Bot, msg: Message) -> ResponseResult<()> {
    send_random_meme_generator(bot, msg).await?;
    Ok(())
}
