Las funciones asincrónicas nos permiten ejecutar código de forma asíncrona\.

Ejemplo en Rust: 
```rust
async fn saluda() {
    println!("¡Hola, mundo!");
}

#[tokio::main]
async fn main() {
    saluda().await;
}
```