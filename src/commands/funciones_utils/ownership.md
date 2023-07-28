El Ownership es un concepto de Rust que nos permite controlar el uso de memoria de nuestro programa\.

Ejemplo en Rust:

```rust
fn main() {
   let s1 = String::from("Hola");
   let s2 = s1;

   println!("El valor de s1 es: {}", s1);
   println!("El valor de s2 es: {}", s2);
}
```
En este ejemplo, la variable s1 tiene el ownership del String "Hola"\.
Luego, al asignar la variable s1 a la variable s2, el ownership del String es transferido a s2, dejando a s1 sin ownership\.
Por lo tanto, al intentar imprimir el valor de s1 se producirá un error\.