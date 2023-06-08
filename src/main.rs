use teloxide::types::Message;
//type Bot = DefaultParseMode<teloxide::Bot>;
// Import the commands and database modules
use crate::commands::*;
pub mod database;
pub mod commands;
pub mod utils;
pub mod buttons;

// Main function that starts the bot
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Initialize the logging environment for Teloxide
    pretty_env_logger::init();
    log::info!("Iniciando Bot...");

    // Load environment variables from a .env file
    dotenv().ok();

    // Connect to the database
    //conectar().await.expect("Error al conectar con la Base de Datos");

    // Print a message to the console to indicate that the bot has started
    println!("
╔═════════════════════════════════════════════════════╗
║                                                     ║
║      Bot Iniciado /-/ Creado por @CrawKatt /-/      ║
║                                                     ║
╚═════════════════════════════════════════════════════╝
\n");

    let bot = teloxide::Bot::from_env().parse_mode(MarkdownV2);

    // Experimental

    let handler = dptree::entry().inspect(|u:Update| {
        println!("{u:#?}");
        //log::info!("{u:?}");
    })
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler))
        .branch(Update::filter_message()
            .branch(Message::filter_new_chat_members().endpoint(
                |bot:Bot, msg:Message| async move {
                    let user = msg.new_chat_members().expect("REASON").get(0).unwrap();

                    bot.send_message(msg.chat.id, format!("Test {:#?}", user))
                        .reply_to_message_id(msg.id)
                        .await?;

                    Ok(())
                },
            ))
        );

    // Funcional a medias (no funcionan los comandos)
/*
    let handler = dptree::entry()
        .map_async(|u: Update| async move {
            println!("{u:#?}");
            respond(())
        })
        .branch(Update::filter_message().endpoint(|bot: teloxide::Bot, msg: Message| async move {
            let user = msg.new_chat_members().expect("REASON").get(0).unwrap();

            bot.send_message(msg.chat.id, format!("Bienvenido al grupo @{}\\!", user.username.clone().unwrap()))
                .reply_to_message_id(msg.id)
                .await?;

            respond(())
        }))
        .branch(Update::filter_callback_query().endpoint(|bot: Bot, query: CallbackQuery| async move {
            // Lógica para manejar los CallbackQuery aquí
            respond(())
        }))
        .branch(Update::filter_inline_query().endpoint(|bot: Bot, query: InlineQuery| async move {
            // Lógica para manejar las InlineQuery aquí
            respond(())
        }));

*/

    // No funciona (this is cursed LOL)
/*
    let handler = Update::filter_message().branch(
        Message::filter_new_chat_members().endpoint(
            |bot:teloxide::Bot, msg:Message| async move {
                let user = msg.new_chat_members().expect("REASON").get(0).unwrap();

                bot.send_message(msg.chat.id, format!("Bienvenido al grupo @{}!", user.username.clone().unwrap()))
                    .reply_to_message_id(msg.id)
                    .await?;
                respond(())
            },
        )
    );
*/

    // Funcional a medias (no funcionan los comandos)
/*
    let handler = Update::filter_message().branch(
        Message::filter_new_chat_members().endpoint(
            |bot:Bot, msg:Message| async move {
                let user = msg.new_chat_members().expect("REASON").get(0).unwrap();

                bot.send_message(msg.chat.id, format!("Bienvenido al grupo @{}\\!", user.username.clone().unwrap()))
                    .reply_to_message_id(msg.id)
                    .await?;
                respond(())
            },
        )
    );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
*/
/*
    // Funcional
    // Create a new `Bot` instance from environment variables and set the message parsing mode to MarkdownV2
    let bot = teloxide::Bot::from_env().parse_mode(MarkdownV2);

    // Call the `repl` function of the `Command` struct with the bot instance and the `action` function
    let handler = dptree::entry().map_async(|u:Update| async move {

        println!("{u:#?}");
        respond(())
    })
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));
*/

    Dispatcher::builder(bot.clone(), handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Command::repl(bot,action).await;

    Ok(())
}

