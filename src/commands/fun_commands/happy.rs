use crate::commands::dependencies::*;

/// # Errors
/// # Panics
pub(crate)  async fn send_happy(bot: Bot, msg: Message) -> ResponseResult<()> {

    let username_author = msg.from().as_ref().and_then(|user| user.username.as_ref());

    let username_author = username_author.map_or("", |username| username);

    let url = nekosbest::get(nekosbest::Category::Happy)
        .await
        .unwrap()
        .url;

    bot.send_animation(msg.chat.id, InputFile::url(url.parse().unwrap()))
        .caption(format!("@{} Está Feliz", username_author))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}
