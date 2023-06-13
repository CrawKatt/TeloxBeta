Los módulos nos permiten organizar nuestro código en diferentes archivos\.

Ejemplo en Rust:

Archivo `funciones.rs`:
```rust
pub fn saludar(nombre: &str) {
    println!("Hola {}", nombre);
}
```
Archivo `main.rs`:
```rust
mod funciones;

fn main() {
    funciones::saludar("Juan");
}
```