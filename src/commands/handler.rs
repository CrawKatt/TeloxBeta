use crate::dependencies::*;

// Derive BotCommands para analizar texto con un comando en este enum.
//
// 1. `rename_rule = "lowercase"` convierte todos los comandos en letras minúsculas.
// 2. `description = "..."` especifica un texto antes de todos los comandos.
//
// Es decir, puede simplemente llamar a Command::descriptions() para obtener una
// descripción de sus comandos en este formato:
// %GENERAL-DESCRIPTION% /// %DESCRIPCIÓN GENERAL%
// %PREFIX%%COMMAND% - %DESCRIPTION% /// %PREFIJO%%COMANDO% - %DESCRIPCIÓN%
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", parse_with = "split")]
pub enum Command {
    Ban,
    Unban,
    Mute,
    Unmute,
    Start,
    Variables,
    Constantes,
    TiposDeDatos,
    Operadores,
    Funciones,
    Arrays,
    Tuplas,
    Vectores,
    Condicionales,
    Loop,
    For,
    While,
    Match,
    Enum,
    Macros,
    Return,
    Metodos,
    Closures,
    Struct,
    Traits,
    Option,
    Result,
    Generics,
    Lifetimes,
    Async,
    Ownership,
    Referencias,
    Borrowing,
    Modulos,
    Shadowing,
    Slices,
    String,
    Iterators,
    Scopes,
    SpamOn,
    Pat,
    Meme,
    Help,
    Novedades,
    Info,
    About,
    Test,
    List,
    Testing,
    SpamOff,
    IfLet,
    Bite,
    Sad,
    Pout,
    Happy,
    Slap,
    Hug,
    Kiss,
    Punch,
    Cuddle,
    Laugh,
    Blush,
    Poke,
    Tickle,
    Feed,
    Highfive,
    Handhold,
    Nom,
    Yeet,
    Kill,
    Smug,
    Kick,
    ThumbsUp,
    Stare,
}

/// # Errors
pub async fn action(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Variables => Box::pin(ejemplos(bot, msg)).await?,
        Command::Constantes => Box::pin(ejemplos(bot, msg)).await?,
        Command::TiposDeDatos => Box::pin(ejemplos(bot, msg)).await?,
        Command::Operadores => ejemplos(bot, msg).await?,
        Command::Arrays => ejemplos(bot, msg).await?,
        Command::Tuplas => ejemplos(bot, msg).await?,
        Command::Vectores => ejemplos(bot, msg).await?,
        Command::Condicionales => ejemplos(bot, msg).await?,
        Command::Loop => ejemplos(bot, msg).await?,
        Command::For => ejemplos(bot, msg).await?,
        Command::While => ejemplos(bot, msg).await?,
        Command::Match => ejemplos(bot, msg).await?,
        Command::Enum => ejemplos(bot, msg).await?,
        Command::Funciones => ejemplos(bot, msg).await?,
        Command::Return => ejemplos(bot, msg).await?,
        Command::Metodos => ejemplos(bot, msg).await?,
        Command::Closures => ejemplos(bot, msg).await?,
        Command::Struct => ejemplos(bot, msg).await?,
        Command::Traits => ejemplos(bot, msg).await?,
        Command::Option => ejemplos(bot, msg).await?,
        Command::Result => ejemplos(bot, msg).await?,
        Command::Generics => ejemplos(bot, msg).await?,
        Command::Lifetimes => ejemplos(bot, msg).await?,
        Command::Macros => ejemplos(bot, msg).await?,
        Command::Ownership => ejemplos(bot, msg).await?,
        Command::Referencias => ejemplos(bot, msg).await?,
        Command::Borrowing => ejemplos(bot, msg).await?,
        Command::Modulos => ejemplos(bot, msg).await?,
        Command::Shadowing => ejemplos(bot, msg).await?,
        Command::Slices => ejemplos(bot, msg).await?,
        Command::String => ejemplos(bot, msg).await?,
        Command::Iterators => ejemplos(bot, msg).await?,
        Command::Scopes => ejemplos(bot, msg).await?,
        Command::Async => ejemplos(bot, msg).await?,
        Command::IfLet => ejemplos(bot, msg).await?,
        Command::About => ejemplos(bot, msg).await?,
        Command::Novedades => ejemplos(bot, msg).await?,

        _ => (),
    };

    Ok(())
}

/// # Errors
pub async fn message(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    let Some(text) = msg.text() else {
        return Ok(());
    };

    match BotCommands::parse(text, me.username()) {
        Ok(Command::Start) => create_buttons(bot, msg).await?,
        Ok(Command::Help) => help_action(bot, msg).await?,

        Ok(Command::Ban) => Box::pin(ban_user(bot, msg)).await?,
        Ok(Command::Unban) => unban_user(bot, msg).await?,
        Ok(Command::Mute) => mute_user_admin(bot, msg).await?,
        Ok(Command::Unmute) => Box::pin(unmute_user(bot, msg)).await?,
        Ok(Command::List) => list_json(bot, msg).await?,
        Ok(Command::Info) => get_chat_member(bot, msg).await?,

        Ok(Command::Meme) => send_random_meme(bot, msg).await?,
        Ok(Command::Pat) => send_pat(bot, msg).await?,
        Ok(Command::Bite) => send_bite(bot, msg).await?,
        Ok(Command::Sad) => send_sad(bot, msg).await?,
        Ok(Command::Pout) => send_pout(bot, msg).await?,
        Ok(Command::Happy) => send_happy(bot, msg).await?,
        Ok(Command::Punch) => send_punch(bot, msg).await?,
        Ok(Command::Slap) => send_slap(bot, msg).await?,
        Ok(Command::Hug) => send_hug(bot, msg).await?,
        Ok(Command::Kiss) => send_kiss(bot, msg).await?,
        Ok(Command::Cuddle) => send_cuddle(bot, msg).await?,
        Ok(Command::Laugh) => send_laugh(bot, msg).await?,
        Ok(Command::Blush) => send_blush(bot, msg).await?,
        Ok(Command::Poke) => send_poke(bot, msg).await?,
        Ok(Command::Kill) => send_kill(bot, msg).await?,
        Ok(Command::Yeet) => send_yeet(bot, msg).await?,
        Ok(Command::Smug) => send_smug(bot, msg).await?,
        Ok(Command::Kick) => send_kick(bot, msg).await?,
        Ok(Command::ThumbsUp) => send_thumbs_up(bot, msg).await?,
        Ok(Command::Stare) => send_stare(bot, msg).await?,

        // Comandos de Anti_Spam (unsafe maldito LOL) >> Anti_Spam Commands (This is
        // cursed LOL) Ok(Command::SpamOn) => handle_command(bot.clone(),
        // msg.clone()).await?, Ok(Command::SpamOff) =>
        // handle_command(bot.clone(), msg.clone()).await?,
        Err(_) => {

            test_json_two(bot.clone(), msg.clone()).await?;

            insert_user_to_sql(&msg)?;

            // if text.contains("https://t.me") {
            // anti_spam(bot.clone(), msg.clone()).await?;
            //}

            // handle_command(bot, msg.clone()).await?;
        }

        _ => action(bot, msg, Command::Variables).await?,
    }

    Ok(())
}

// async fn handle_command(bot: Bot, message: Message) -> ResponseResult<()> {
// if let Some(text) = message.text() {
// match text {
// "/spam_on" => {
// unsafe { ANTI_SPAM_ENABLED = true };
// bot.send_message(message.chat.id, "Anti-spam
// activado").parse_mode(ParseMode::Html).await?; }
// "/spam_off" => {
// unsafe { ANTI_SPAM_ENABLED = false };
// bot.send_message(message.chat.id, "Anti-spam
// desactivado").parse_mode(ParseMode::Html).await?; }
// _ => (),
// }
// }
// Ok(())
// }
