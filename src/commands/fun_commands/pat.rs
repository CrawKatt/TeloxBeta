use crate::commands::admin_commands::*;

pub async fn send_pat(bot: Bot, msg: Message) -> ResponseResult<()> {
    random_pat(bot, msg).await?;
    Ok(())
}