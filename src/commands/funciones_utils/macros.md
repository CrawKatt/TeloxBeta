Las macros nos permiten escribir código que produce código\.

Ejemplo en Rust:
```
macro_rules! say_hello {
    () => (
        println!("Hola");
    );
}

fn main() {
    say_hello!();
}
```