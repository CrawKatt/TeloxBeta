Los Tiempos de Vida son un concepto de Rust que nos permite controlar el tiempo de vida de las referencias\.

Ejemplo en Rust:
```
fn main() {
    let r; {
        let x = 5;
        r = &x;
    }
    println!("El valor de r es: {}", r);
}
```