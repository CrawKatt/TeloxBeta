Las funciones asincrónicas nos permiten ejecutar código de forma asincrónica\.

Ejemplo en Rust: 
```
async fn saluda() {
    println!("¡Hola, mundo!");
}

#[tokio::main]
async fn main() {
    saluda().await;
}
```