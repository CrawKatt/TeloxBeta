use crate::commands::dependencies::*;

/// # Errors
/// # Panics
pub(crate) async fn send_laugh(bot: Bot, msg: Message) -> ResponseResult<()> {

    let username_author = msg.from().as_ref().and_then(|user| user.username.as_ref());

    let username_author = username_author.map_or("", |username| username);

    let url = nekosbest::get(nekosbest::Category::Laugh)
        .await
        .unwrap()
        .url;

    bot.send_animation(msg.chat.id, InputFile::url(url.parse().unwrap()))
        .caption(format!("@{} Se ríe a carcajadas", username_author))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}
