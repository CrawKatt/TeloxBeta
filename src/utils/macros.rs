#[macro_export]
macro_rules! username_changed {
    ($bot:ident, $chat_id:expr, $user:expr, $db_username:expr, $username:expr) => {

        $bot.send_message(
            $chat_id,
            format!(
                "El usuario {} cambió su nombre de usuario de {} a {}.",
                $user.first_name, $db_username, $username
            ),
        )
        .parse_mode(ParseMode::Html)
        .await?;
    };
}

#[macro_export]
macro_rules! first_name_changed {
    ($bot:expr, $chat_id:expr, $username:expr, $db_name:expr, $new_name:expr) => {

        $bot.send_message(
            $chat_id,
            format!(
                "El usuario {} cambió su nombre de [{}] a [{}].",
                $username, $db_name, $new_name
            ),
        )
        .parse_mode(ParseMode::Html)
        .await?;
    };
}

#[macro_export]
macro_rules! last_name_changed {
    ($bot:expr, $chat_id:expr, $username:expr, $db_last_name:expr, $new_last_name:expr) => {

        $bot.send_message(
            $chat_id,
            format!(
                "El usuario {:} cambió su apellido de [{:?}] a [{:?}].",
                $username, $db_last_name, $new_last_name
            ),
        )
        .parse_mode(ParseMode::Html)
        .await?;
    };
}
