El tipo de dato `Result<T, E>` es un tipo de dato que puede ser alguno de dos valores: `Ok(T)` o `Err(E)`\.

Ejemplo en Rust:

```rust
fn main() {
    let valor: Result<i32, &str> = Ok(5);
    let valor2: Result<i32, &str> = Err("Error");
}
```