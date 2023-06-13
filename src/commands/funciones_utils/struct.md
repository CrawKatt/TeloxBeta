Los structs son tipos de datos personalizados que nos permiten agrupar diferentes valores en un solo tipo\.

Ejemplo en Rust:

```rust
struct Rectángulo {
   ancho: u32,
   alto: u32,
}

fn main() {
   let rectangulo = Rectangulo {
     ancho: 30,
     alto: 50,
   };

   println!("El área del rectángulo es: {}",
   rectangulo.ancho * rectangulo.alto);
}
```