Los traits son como interfaces en otros lenguajes de programación, nos permiten definir comportamientos comunes a diferentes tipos de datos\.

Ejemplo en Rust:

```
trait Suma {
    fn suma(&self) -> i32;
}

struct Rectangulo {
    ancho: i32,
    alto: i32,
}

impl Suma for Rectangulo {
    fn suma(&self) -> i32 {
        self.ancho + self.alto
    }
}

fn main() {
    let rectangulo = Rectangulo {
        ancho: 30,
        alto: 50,
    };

    println!("El área del rectángulo es: {}", rectangulo.suma());
}
```