El match es una expresión que nos permite comparar un valor con una serie de patrones y ejecutar un bloque de código cuando el patrón coincide con el valor\.

Ejemplo en Rust:
```rust
let numero = 13;

match numero {
    1 => println!("Uno"),
    2 => println!("Dos"),
    3 => println!("Tres"),
    4 => println!("Cuatro"),
    5 => println!("Cinco"),
    _ => println!("Otro número"),
}
```