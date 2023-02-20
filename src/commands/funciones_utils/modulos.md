Los módulos nos permiten organizar nuestro código en diferentes archivos\.

Ejemplo en Rust:

Archivo `funciones.rs`:
```
pub fn saludar(nombre: &str) {
    println!("Hola {}", nombre);
}
```
Archivo `main.rs`:
```
mod funciones;

fn main() {
    funciones::saludar("Juan");
}
```