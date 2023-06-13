Los Strings son una cadena de carácteres que representan texto\.

Ejemplo en Rust:

```rust
fn main() { 
   let mut texto = String::new(); 

   let data = "Hola"; 

   let texto = data.to_string(); 

   let texto = " mundo".to_string(); 

   let texto = String::from("Hola"); 
}
```