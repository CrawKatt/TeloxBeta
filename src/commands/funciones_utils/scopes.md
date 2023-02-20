Los Scopes nos permiten controlar la visibilidad de los elementos\.

Ejemplo en Rust:

```
fn main() { 
   let x = 5; 

   { 
      let y = 10; 
      println!("El valor de x es: {} y el valor de y es: {}", x, y); 
   } 

   // la variable y ya no es visible en este ámbito
   // Esto dará un error en tiempo de compilación
   println!("El valor de x es: {} y el valor de y es: {}", x, y); 
}
```