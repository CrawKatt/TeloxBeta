Los Iteradores son un tipo de dato que nos permiten iterar sobre una colección de datos\.

Ejemplo en Rust:
```
let numeros = vec![1, 2, 3, 4, 5];
    for numero in numeros.iter() {
        println!("El valor de número: {:?}", numero);
    }
```