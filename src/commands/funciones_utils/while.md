El bucle while se usa para ejecutar un bloque de código mientras una condición sea verdadera\.

Ejemplo en Rust:
```rust
fn main() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        }

        println!("x = {}", x);
    }
}
```