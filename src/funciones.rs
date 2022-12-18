use teloxide::prelude::Requester;

use crate::comandos::MyBot;
use crate::{Message};
use crate::ResponseResult;


pub async fn variables(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Variables: Son un espacio en memoria cuyo valor puede asignarse y cambiar \n\nEjemplo en Rust: \n`let mi_variable = \"valor\":`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn constantes(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Constantes: Son un espacio en memoria cuyo valor no puede cambiar \n\nEjemplo en Rust: \n`const MI_CONSTANTE: i32 = 10;`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn tipos_de_datos(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Tipos de Datos: Las variables se definen con un tipo de dato que puede ser: \n\nUn número entero \nUn número Flotante/Decimal \nUn numero negativo \nUn String/Cadena \\(Palabra o letra\\), etc \n\nEjemplo en Rust: \ni8,i16,i32,i64,i128 \\= Tipo Entero \nu8,u16,u32,u64,u128 \\= Tipo Entero \\(Solo números positivos\\)").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn operadores(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Operadores: En programación, tenemos distintos tipos de operadores para manejar datos en nuestras variables\\. Entre estos están: \n\n`//Los Operadores Básicos: \n\n+ Suma \n\n- Resta \n\n* Multiplicación \n\n/ Division \n\n% División (Con resto/residuo) \n\n//Los Operadores Relacionales: \n\n> Mayor que \n\n< Menor que \n\n>= Mayor o igual que \n\n<= Menor o igual que \n\n== Igual \n\n!= Diferente de \n\n//Los Operadores Lógicos \n\n|| Or (o) \n\n&& And (y)`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn arrays(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "un Arreglo/Array nos permite almacenar múltiples valores dentro de una colección\\. En Rust, un Arreglo/Array debe almacenar el mismo tipo de dato \\(Solo enteros, Solo Strings, Solo Booleanos, etc\\) \n\nEjemplo en Rust: \n`let array = [1, 2, 3, 4, 5];` \n\nConsejo: En Rust, los Arreglos/Arrays se rigen por la regla de los indices\\. A cada elemento le corresponde un indice y los indices comienzan en cero\\. Si tomamos nuestro ejemplo el índice en dicho ejemplo es: \n\n`0 -> 1 \n1 -> 2 \n2 -> 3 \n3 -> 4 \n4 -> 5`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn tuplas(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Las Tuplas son similares a los Arreglos/Arrays, pero con la diferencia de que pueden almacenar distintos tipos de datos\\. En Rust, las Tuplas se definen con paréntesis y separando cada dato con una coma \n\nEjemplo en Rust: \n`let tupla = (1, 2, 3, 4, 5);` \n\nConsejo: En Rust, las Tuplas se rigen por la regla de los indices\\. A cada elemento le corresponde un indice y los indices comienzan en cero\\. Si tomamos nuestro ejemplo el índice en dicho ejemplo es: \n\n`0 -> 1 \n1 -> 2 \n2 -> 3 \n3 -> 4 \n4 -> 5`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn vectores(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Los Vectores son similares a los Arreglos/Arrays, pero con la diferencia de que pueden almacenar distintos tipos de datos\\. En Rust, los Vectores se definen con la palabra reservada `vec!` y separando cada dato con una coma \n\nEjemplo en Rust: \n`let vector = vec![1, 2, 3, 4, 5];` \n\nConsejo: En Rust, los Vectores se rigen por la regla de los indices\\. A cada elemento le corresponde un indice y los indices comienzan en cero\\. Si tomamos nuestro ejemplo el índice en dicho ejemplo es: \n\n`0 -> 1 \n1 -> 2 \n2 -> 3 \n3 -> 4 \n4 -> 5`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn condicionales(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Son grupos de sentencias o sentencias individuales que te permiten condicionar la decisión entre la elección de una opción y otra\\. \n\nEjemplo en Rust: \n`let color = \"verde\"; \n\nif color == \"Verde\" {\n   println!(\"Puede continuar.\"); \n} else { \n   println!(\"El color no es verde\"); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn loops(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "El bucle es una estructura de control que nos permite repetir un bloque de código tantas veces como sea necesario\\. \n\nEjemplo en Rust: \n`fn main() { \n   let mut contador = 0; \n\n   loop { \n      contador += 1; \n\n      if contador == 10 { \n         break; \n      } \n   } \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn fors(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "En Rust, el ciclo for nos permitirá iterar sobre una colección de datos\\. Ya sea un vector, un Arreglo/Array, una tupla, etc\\. El ciclo for funcionara como un for each \n\nEjemplo en Rust: \n`let numeros : [i32; 5] = [1, 2, 3, 4, 5]; \n\nfor numero in numeros.iter( ) {\n    println!(\"El valor de número: {:?}\", numero); \n}` \n\nEjemplo de algoritmo Fizz Buzz utilizando el ciclo for en Rust: \n\n`for numero in 1..101 {\n\n    if numero % 3 == 0 && numero % 5 == 0 {\n    println!(\"Fizz Buzz\"); \n\n} else if numero % 3 == 0 { \n    println!(\"Fizz\"); \n\n} else if numero % 5 == 0 {\n    println!(\"Buzz\"); \n\n} else {\n    println!(\"{}\", numero); \n   }\n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn whiles(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "El bucle while se usa para ejecutar un bloque de código mientras una condición sea verdadera\\. \n\nEjemplo en Rust: \n`fn main() { \n   let mut x = 1; \n\n   while x < 1000 { \n      x *= 2; \n\n      if x == 64 { continue; } \n\n      println!(\"x = {}\", x); \n   } \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn matchs(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "El match es una expresión que nos permite comparar un valor con una serie de patrones y ejecutar un bloque de código cuando el patrón coincide con el valor\\. \n\nEjemplo en Rust: \n`let numero = 13; \n\nmatch numero { \n   1 => println!(\"Uno\"), \n   2 => println!(\"Dos\"), \n   3 => println!(\"Tres\"), \n   4 => println!(\"Cuatro\"), \n   5 => println!(\"Cinco\"), \n   _ => println!(\"Otro número\"), \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn enums(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Un Enum es un tipo que almacena diferentes variantes, almacena diferentes opciones\\. \n\nEjemplo en Rust: \n`enum Response {\n    Sucess, // Se completó correctamente \n    Error(u32, String), // Podemos indicar un código de Error a través de una Tupla \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn funciones(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Las funciones son bloques de código que se pueden reutilizar en diferentes partes de nuestro programa\\. \n\nEjemplo en Rust: \n`fn saludar(nombre: &str) {\n   println!(\"Hola {}\", nombre); \n} \n\nfn main() {\n   saludar(\"Juan\"); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn metodos(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Los métodos son similares a las funciones, pero se diferencian en que los métodos se definen dentro de un contexto, como una estructura o un Enum\\. \n\nEjemplo en Rust: \n`struct Rectángulo { \n   ancho: u32, \n   alto: u32, \n} \n\nimpl Rectángulo { \n   fn area(&self) -> u32 { \n      self.ancho * self.alto \n      } \n} \n\nfn main() { \n   let rectangulo = Rectangulo { \n     ancho: 30, \n     alto: 50, \n   }; \n\n   println!(\"El área del rectángulo es: {}\", \n   rectángulo.area()); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn returns(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Las funciones pueden devolver valores, para ello se utiliza la palabra reservada return\\. \n\nEjemplo en Rust: \n`fn suma(a: i32, b: i32) -> i32 { \n   return a + b; \n} \n\nfn main() { \n   let resultado = suma(5, 5); \n   println!(\"El resultado es: {}\", resultado); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn estructuras(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Las estructuras son tipos de datos personalizados que nos permiten agrupar diferentes valores en un solo tipo\\. \n\nEjemplo en Rust: \n`struct Rectángulo { \n   ancho: u32, \n   alto: u32, \n} \n\nfn main() { \n   let rectangulo = Rectangulo { \n     ancho: 30, \n     alto: 50, \n   }; \n\n   println!(\"El área del rectángulo es: {}\", \n   rectángulo.ancho * rectángulo.alto); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn traits(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Los traits son como interfaces en otros lenguajes de programación, nos permiten definir comportamientos comunes a diferentes tipos de datos\\. \n\nEjemplo en Rust: \n`trait Suma { \n   fn suma(&self) -> i32; \n} \n\nstruct Rectángulo { \n   ancho: i32, \n   alto: i32, \n} \n\nimpl Suma for Rectángulo { \n   fn suma(&self) -> i32 { \n      self.ancho + self.alto \n   } \n} \n\nfn main() { \n   let rectangulo = Rectángulo { \n     ancho: 30, \n     alto: 50, \n   }; \n\n   println!(\"El área del rectángulo es: {}\", \n   rectángulo.suma()); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn closures(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Las Closures son funciones anónimas que se pueden almacenar en variables o pasar como argumentos a otras funciones\\. \n\nEjemplo en Rust: \n`let suma = |a: i32, b: i32| -> i32 { \n   a + b \n}; \n\nfn main() { \n   let resultado = suma(5, 5); \n   println!(\"El resultado es: {}\", resultado); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn generics(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Los generics nos permiten crear tipos de datos genéricos, que pueden ser de cualquier tipo\\. \n\nEjemplo en Rust: \n`struct Generic<T> { \n   valor: T, \n} \n\nfn main() { \n   let entero = Generico { \n     valor: 5, \n   }; \n\n   let flotante = Generico { \n     valor: 5.0, \n   }; \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn option(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "El tipo de dato Option\\<T\\> es un tipo de dato que puede ser alguno de dos valores: Some\\(T\\) o None\\. \n\nEjemplo en Rust: \n`fn main() { \n   let valor: Option<i32> = Some(5); \n   let valor2: Option<i32> = None; \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn result(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "El tipo de dato Result<T, E> es un tipo de dato que puede ser alguno de dos valores: Ok(T) o Err(E)\\. \n\nEjemplo en Rust: \n`fn main() { \n   let valor: Result<i32, &str> = Ok(5); \n   let valor2: Result<i32, &str> = Err(\"Error\"); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn iterators(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Los Iteradores son un tipo de dato que nos permiten iterar sobre una colección de datos\\. \n\nEjemplo en Rust: \n`let numeros = vec![1, 2, 3, 4, 5]; \n\nfor numero in numeros.iter() { \n   println!(\"El valor de número: {:?}\", numero); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn shadowing(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "El Shadowing nos permite reutilizar el nombre de una variable con un nuevo valor\\. \n\nEjemplo en Rust: \n`fn main() { \n   let x = 5; \n\n   let x = x + 1; \n\n   let x = x * 2; \n\n   println!(\"El valor de x es: {}\", x); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn ownership(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "El Ownership es un concepto de Rust que nos permite controlar el uso de memoria de nuestro programa\\. \n\nEjemplo en Rust: \n`fn main() { \n   let s1 = String::from(\"Hola\"); \n   let s2 = s1; \n\n   println!(\"El valor de s1 es: {}\", s1); \n   println!(\"El valor de s2 es: {}\", s2); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn referencias(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Las Referencias nos permiten tener múltiples referencias a un valor sin que el valor se mueva\\. \n\nEjemplo en Rust: \n`fn main() { \n   let s1 = String::from(\"Hola\"); \n   let len = calcula_longitud(&s1); \n\n   println!(\"La longitud de s1 es: {}\", len); \n}` \n\n`fn calcula_longitud(s: &String) -> usize { \n   s.len() \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn borrowing(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "El préstamo es una característica de Rust que permite que un valor sea prestado a una función, método u otro valor\\. \n\nEjemplo en Rust: \n`fn main() { \n   let mut s = String::from(\"Hola\"); \n\n   cambiar(&mut s); \n\n   println!(\"El valor de s es: {}\", s); \n} \n\nfn cambiar(s: &mut String) { \n   s.push_str(\", mundo\"); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn slices(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Los Slices nos permiten referenciar una secuencia de elementos de una colección\\. \n\nEjemplo en Rust: \n`fn main() { \n   let s = String::from(\"Hola, mundo\"); \n\n   let hola = &s[0..5]; \n   let mundo = &s[7..12]; \n\n   println!(\"El valor de hola es: {}\", hola); \n   println!(\"El valor de mundo es: {}\", mundo); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn modulos(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Los módulos nos permiten organizar nuestro código en diferentes archivos\\. \n\nEjemplo en Rust: \n`// main.rs \nmod funciones; \n\nfn main() {\n   funciones::saludar(\"Juan\"); \n}` \n\n`// funciones.rs \npub fn saludar(nombre: &str) {\n   println!(\"Hola {}\", nombre); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn strings(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Las cadenas son una estructura de datos que almacena una secuencia de caracteres\\. \n\nEjemplo en Rust: \n`fn main() { \n   let mut s = String::new(); \n\n   let datos = \"Hola\"; \n\n   let s = datos.to_string(); \n\n   let s = `").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn lifetimes(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Los Tiempos de Vida son un concepto de Rust que nos permite controlar el tiempo de vida de las referencias\\. \n\nEjemplo en Rust: \n`fn main() { \n   let r; \n\n   { \n      let x = 5; \n      r = &x; \n   } \n\n   println!(\"El valor de r es: {}\", r); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn macros(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Los macros nos permiten escribir código que produce código\\. \n\nEjemplo en Rust: \n`macro_rules! say_hello { \n   () => ( \n      println!(\"Hola\"); \n   ) \n} \n\nfn main() { \n   say_hello!(); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn asyncs(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Las funciones asincrónicas nos permiten ejecutar código de forma asincrónica\\. \n\nEjemplo en Rust: \n`async fn main() { \n   let respuesta = reqwest::get(\"https://www.rust-lang.org\") \n      .await \n      .unwrap() \n      .text() \n      .await \n      .unwrap(); \n\n   println!(\"El tamaño de la respuesta es: {}\", respuesta.len()); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn scopes(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Los Scopes nos permiten controlar la visibilidad de los elementos\\. \n\nEjemplo en Rust: \n`fn main() { \n   let x = 5; \n\n   { \n      let y = 10; \n      println!(\"El valor de x es: {} y el valor de y es: {}\", x, y); \n   } \n\n   println!(\"El valor de x es: {} y el valor de y es: {}\", x, y); \n}`").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn novedades(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Cambios de la Versión 1\\.2 \n\n*Añadido:* Comando /unmute que remueve el silencio a un usuario silenciado \n\n*Uso:* /unmute respondiendo un mensaje del usuario silenciado \n\n*Añadido:* Más memes a los assets del Bot para mayor variedad del comando /meme \n\n*Añadido:* Más videos y gifs a los assets del Bot para mayor variedad al banear a un usuario con /ban \n\n*Removido:* Comando /image creado solo con fines de debug y test de las funciones del Bot \n\n*Removido:* Comando /gif creado solo con fines de debug y test de las funciones del Bot \n\n*Removido:* Comando /video creado solo con fines de debug y test de las funciones del Bot").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn about(bot: MyBot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Bot creado por @CrawKatt \n\nGitHub: \nhttps://github\\.com/CrawKatt \n\nVersion del Bot: 1\\.2").await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub async fn get_chat_administrators(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let chat_administrators = bot.get_chat_administrators(msg.chat.id).await?;
    println!("{:?}", chat_administrators);

    bot.send_message(msg.chat.id, format!("`Chat Administrators: {:?}`", chat_administrators)).await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

// get @username from message from command and async function
pub async fn get_username(bot: MyBot, msg: Message) -> ResponseResult<()> {

    let user = msg.reply_to_message().unwrap().from().unwrap();

    let username_user = user.clone().username;
    println!("@username : {:?}", username_user);

    bot.send_message(msg.chat.id, format!("Tu @Username es : @{}", username_user.unwrap())).await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    return Ok(())
}
