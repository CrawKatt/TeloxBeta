use crate::utils::dependencies::*;

pub mod buttons;
pub mod commands;
pub mod database;
pub mod utils;

type MemberResult = Result<(), Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    pretty_env_logger::init();

    log::info!("Iniciando Bot...");

    dotenv().ok();

    // Connect to the database
    // conectar().await.expect("Error al conectar con la Base de Datos");

    let init_message = include_str!("init_message.txt");

    println!("{init_message}\n");

    let bot = teloxide::Bot::from_env().parse_mode(ParseMode::Html);

    let handler = dptree::entry()
        .inspect(|_u: Update| {
            // println!("{u:#?}");
        })
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler))
        .branch(
            Update::filter_chat_member()
                .filter(|m: ChatMemberUpdated| {

                    m.old_chat_member.kind.is_left() && m.new_chat_member.kind.is_present()
                        || m.old_chat_member.kind.is_present() && m.new_chat_member.kind.is_left()
                })
                .endpoint(chat_member_welcome),
        );

    // We create a dispatcher for our bot
    Dispatcher::builder(bot.clone(), handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

/// Welcome Function
/// We use `ChatMemberUpdated` instead of Message for our function because
/// Chat member updates != messages
async fn chat_member_welcome(bot: Bot, chat_member: ChatMemberUpdated) -> MemberResult {

    // We use this variable for get the user
    let user = chat_member.new_chat_member.user;

    // We use this variable for get the user_id
    let user_id = chat_member.old_chat_member.user.id;

    // We use this variable for get the group name
    let telegram_group_name = chat_member.chat.title().unwrap_or("");

    // We use this variable for get the status of the user and filter if the user is
    // present or is gone
    let chat_member_status = chat_member.old_chat_member;

    // We converts the user_id to i64 type
    let ChatId(user_id) = ChatId::from(user_id);

    // We get the full_name of the user via `mention()` method and we use
    // `unwrap_or_else` for get the first_name via `full_name` method
    // if the user don't have a username
    let username = user
        .mention()
        .unwrap_or_else(|| html::user_mention(user_id, user.full_name().as_str()));

    if chat_member_status.is_present() {

        bot.send_message(chat_member.chat.id, format!("Hasta pronto {username}!"))
            .await?;
    }

    if !chat_member_status.is_present() {

        bot.send_message(
            chat_member.chat.id,
            format!("Bienvenido a {telegram_group_name} {username}!"),
        )
        .await?;
    }

    Ok(())
}
