En Rust, el ciclo for nos permitirá iterar sobre una colección de datos\. Ya sea un vector, un Arreglo/Array, una tupla, etc\.

El ciclo for funcionará como un for each

Ejemplo en Rust:
`
let numeros : [i32; 5] = [1, 2, 3, 4, 5];
    for numero in numeros.iter( ) {
        println!("El valor de número: {:?}", numero);
    }
`

Ejemplo de algoritmo Fizz Buzz utilizando el ciclo for en Rust:
`
for numero in 1..101 {
    if numero % 3 == 0 && numero % 5 == 0 {
        println!("Fizz Buzz");
    } else if numero % 3 == 0 {
        println!("Fizz"); 
    } else if numero % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", numero);
    }
}
`