use crate::commands::*;

// enum for the examples of the bot
enum Ejemplo {
    About,      Arrays,     Async,      Borrowing,   Closures,   Condicionales,  Constantes,  Enum,
    For,        Funciones,  Generics,   Iterators,   Lifetimes,  Loop,           Macros,      Match,
    Metodos,    Modulos,    Novedades,  Operadores,  Option,     Ownership,      Result,      Return,
    Scopes,     Shadowing,  Slices,     String,      Struct,     TiposDeDatos,   Traits,      Tuplas,
    Variables,  Vectores,   While,
}

// examples of the use of the bot
pub async fn ejemplos(bot: Bot, msg: Message) -> ResponseResult<()> {
    let ejemplo =
        match msg.text() {
        Some("/variables")      => Ejemplo::Variables,     Some("/constantes") => Ejemplo::Constantes,
        Some("/tiposdedatos")   => Ejemplo::TiposDeDatos,  Some("/about")      => Ejemplo::About,
        Some("/arrays")         => Ejemplo::Arrays,        Some("/async")      => Ejemplo::Async,
        Some("/borrowing")      => Ejemplo::Borrowing,     Some("/closures")   => Ejemplo::Closures,
        Some("/condicionales")  => Ejemplo::Condicionales, Some("/enum")       => Ejemplo::Enum,
        Some("/for")            => Ejemplo::For,           Some("/funciones")  => Ejemplo::Funciones,
        Some("/generics")       => Ejemplo::Generics,      Some("/iterators")  => Ejemplo::Iterators,
        Some("/lifetimes")      => Ejemplo::Lifetimes,     Some("/loop")       => Ejemplo::Loop,
        Some("/macros")         => Ejemplo::Macros,        Some("/match")      => Ejemplo::Match,
        Some("/metodos")        => Ejemplo::Metodos,       Some("/modulos")    => Ejemplo::Modulos,
        Some("/novedades")      => Ejemplo::Novedades,     Some("/operadores") => Ejemplo::Operadores,
        Some("/option")         => Ejemplo::Option,        Some("/ownership")  => Ejemplo::Ownership,
        Some("/result")         => Ejemplo::Result,        Some("/return")     => Ejemplo::Return,
        Some("/scopes")         => Ejemplo::Scopes,        Some("/shadowing")  => Ejemplo::Shadowing,
        Some("/slices")         => Ejemplo::Slices,        Some("/string")     => Ejemplo::String,
        Some("/struct")         => Ejemplo::Struct,        Some("/traits")     => Ejemplo::Traits,
        Some("/tuplas")         => Ejemplo::Tuplas,        Some("/vectores")   => Ejemplo::Vectores,
        Some("/while")          => Ejemplo::While,
        _ => {
            bot.send_message(msg.chat.id, format!("Comando desconocido `{:#?}`", msg.text().unwrap())).await?;
            return Ok(());
        }
    };

    // Call MarkDown file example
    let text =
        match ejemplo {
        Ejemplo::Variables      => include_str!("funciones_utils/variables.md"),      Ejemplo::Constantes  => include_str!("funciones_utils/constantes.md"),
        Ejemplo::TiposDeDatos   => include_str!("funciones_utils/tipos_de_datos.md"), Ejemplo::About       => include_str!("funciones_utils/about.md"),
        Ejemplo::Arrays         => include_str!("funciones_utils/arrays.md"),         Ejemplo::Async       => include_str!("funciones_utils/async.md"),
        Ejemplo::Borrowing      => include_str!("funciones_utils/borrowing.md"),      Ejemplo::Closures    => include_str!("funciones_utils/closures.md"),
        Ejemplo::Condicionales  => include_str!("funciones_utils/condicionales.md"),  Ejemplo::Enum        => include_str!("funciones_utils/enum.md"),
        Ejemplo::For            => include_str!("funciones_utils/for.md"),            Ejemplo::Funciones   => include_str!("funciones_utils/funciones.md"),
        Ejemplo::Generics       => include_str!("funciones_utils/generics.md"),       Ejemplo::Iterators   => include_str!("funciones_utils/iterators.md"),
        Ejemplo::Lifetimes      => include_str!("funciones_utils/lifetimes.md"),      Ejemplo::Loop        => include_str!("funciones_utils/loop.md"),
        Ejemplo::Macros         => include_str!("funciones_utils/macros.md"),         Ejemplo::Match       => include_str!("funciones_utils/match.md"),
        Ejemplo::Metodos        => include_str!("funciones_utils/metodos.md"),        Ejemplo::Modulos     => include_str!("funciones_utils/modulos.md"),
        Ejemplo::Novedades      => include_str!("funciones_utils/novedades.md"),      Ejemplo::Operadores  => include_str!("funciones_utils/operadores.md"),
        Ejemplo::Option         => include_str!("funciones_utils/option.md"),         Ejemplo::Ownership   => include_str!("funciones_utils/ownership.md"),
        Ejemplo::Result         => include_str!("funciones_utils/result.md"),         Ejemplo::Return      => include_str!("funciones_utils/return.md"),
        Ejemplo::Scopes         => include_str!("funciones_utils/scopes.md"),         Ejemplo::Shadowing   => include_str!("funciones_utils/shadowing.md"),
        Ejemplo::Slices         => include_str!("funciones_utils/slices.md"),         Ejemplo::String      => include_str!("funciones_utils/string.md"),
        Ejemplo::Struct         => include_str!("funciones_utils/struct.md"),         Ejemplo::Traits      => include_str!("funciones_utils/traits.md"),
        Ejemplo::Tuplas         => include_str!("funciones_utils/tuplas.md"),         Ejemplo::Vectores    => include_str!("funciones_utils/vectores.md"),
        Ejemplo::While          => include_str!("funciones_utils/while.md"),
    };

    bot.send_message(msg.chat.id, text).await?;
    bot.delete_message(msg.chat.id, msg.id).await?;
    println!("JSON Info: \n{:#?}", msg);

    Ok(())
}



