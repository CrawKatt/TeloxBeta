Las funciones pueden devolver valores en Rust utilizando la palabra reservada return\.

Ejemplo en Rust:
```rust
fn suma(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let resultado = suma(5, 5);
    println!("El resultado es: {}", resultado);
}
```