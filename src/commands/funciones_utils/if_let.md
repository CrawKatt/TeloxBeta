If let se utiliza para desempaquetar y hacer coincidir un valor de un tipo Option\<T\> o Result de forma concisa\.

Ejemplo en Rust:
```rust
fn main() {
    let option_value: Option<i32> = Some(5);

    if let Some(x) = option_value {
        println!("El valor es: {}", x);
    } else {
        println!("La opción no contiene un valor.");
    }
}
```