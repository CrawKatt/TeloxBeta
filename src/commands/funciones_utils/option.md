El tipo de dato `Option<T>` es un tipo de dato que puede ser alguno de dos valores: `Some(T)`\(Hay valor\) o None \(Sin valor\)\.

Ejemplo en Rust:

```rust
fn main() {
    let valor: Option<i32> = Some(5);
    let valor2: Option<i32> = None;
}
```

Consejo: En Rust, no existe el `Null` y se utiliza `None` para indicar que no hay valor\.