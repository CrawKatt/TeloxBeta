Un Enum es un tipo que almacena diferentes variantes, almacena diferentes opciones\.

Ejemplo en Rust:
```
enum Response {
    Sucess, // Se completó correctamente 
    Error(u32, String), // Podemos indicar un código de Error a través de una Tupla 
}
```