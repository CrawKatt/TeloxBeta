Los generics nos permiten crear tipos de datos genéricos, que pueden ser de cualquier tipo\.

Ejemplo en Rust:
```
struct Generic<T> {
    valor: T,
} 

fn main() {
    let entero = Generico {valor: 5,};
    let flotante = Generico {valor: 5.0,};
}
```