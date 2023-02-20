El tipo de dato `Option<T>` es un tipo de dato que puede ser alguno de dos valores: `Some(T)` o None\.

Ejemplo en Rust:

```
fn main() {
    let valor: Option<i32> = Some(5);
    let valor2: Option<i32> = None;
}
```