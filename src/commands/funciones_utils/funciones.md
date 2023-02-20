Las funciones son bloques de código que se pueden reutilizar en diferentes partes de nuestro programa\.

Ejemplo en Rust:
```
fn saludar(nombre: &str) {
    println!("Hola {}", nombre);
}

fn main() {
    saludar("Juan");
}
```