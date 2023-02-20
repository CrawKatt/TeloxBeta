Las cadenas de caracteres son una estructura de datos que nos permite almacenar texto\.

Ejemplo en Rust:

```
fn main() { 
   let mut texto = String::new(); 

   let data = "Hola"; 

   let texto = data.to_string(); 

   let texto = " mundo".to_string(); 

   let texto = String::from("Hola"); 
}
```