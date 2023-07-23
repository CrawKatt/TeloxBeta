use crate::commands::admin_commands::*;

pub async fn send_kill(bot: Bot, msg: Message) -> ResponseResult<()> {
    let Some(text) = msg.text() else {
        return Ok(());
    };

    let (_, username_target) = match text.find(' ') {
        Some(index) => text.split_at(index),
        None => ("", text),
    };

    let username_author = match msg.from().as_ref() {
        Some(user) => user.username.as_ref(),
        None => None,
    };

    let username_author = match username_author {
        Some(username) => username,
        None => "",
    };

    let url = nekosbest::get(nekosbest::Category::Shoot).await.unwrap().url;
    bot.send_animation(msg.chat.id, InputFile::url(url.parse().unwrap()))
        .caption(format!("@{} Está abrazando a{}", username_author, username_target))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}