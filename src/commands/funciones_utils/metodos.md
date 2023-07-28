Los métodos son similares a las funciones, pero se diferencian en que los métodos se definen dentro de un contexto, como una estructura o un Enum\.

Ejemplo en Rust:
```rust
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }
}

fn main() {
    let rectangulo = Rectangulo {
        ancho: 30,
        alto: 50,
    };
    println!("El área del rectángulo es: {}", rectangulo.area());
}
```