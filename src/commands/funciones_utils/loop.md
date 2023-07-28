El bucle es una estructura de control que nos permite repetir un bloque de código tantas veces como sea necesario\. \

Ejemplo en Rust:
```rust
fn main() {
    let mut contador = 0;
    loop {
        contador += 1;
        if contador == 10 {
            break;
        }
    }
}
```