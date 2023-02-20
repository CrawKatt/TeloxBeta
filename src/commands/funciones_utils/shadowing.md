El Shadowing nos permite reutilizar el nombre de una variable con un nuevo valor\.

Ejemplo en Rust:

```
fn main() { 
    let x = 5; 

    let x = x + 1; 

    let x = x * 2; 

    println!("El valor de x es: {}", x); 
}
```