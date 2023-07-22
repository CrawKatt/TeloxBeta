use crate::commands::admin_commands::*;

pub async fn send_bite(bot: Bot, msg: Message) -> ResponseResult<()> {
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

    let random_pat = nekosbest::get(nekosbest::Category::Bite).await.unwrap().url;
    bot.send_animation(msg.chat.id, InputFile::url(random_pat.parse().unwrap()))
        .caption(format!("@{} Ha mordido a{}", username_author, username_target))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}