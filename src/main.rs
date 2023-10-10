//// 2023/10/11 // 1:18 //

//// #21 Argumentos (línea de comandos)

// Los argumentos de la línea de comandos son parámetros que se incluyen en la línea de
// comandos (también conocida como terminal o shell), después de lnombre del programa que
// vamos a ejecutar y que añaden información al programa para su ejecución.

// Linux
// ls -a /home/luis

// Windows
// dir *.txt c:\temp


use std::env;

fn main() {

    // let argumentos: Vec<String> = env::args().collect();
    // dbg!(argumentos);


    // let argumentos: Vec<String> = env::args().collect();

    // let parametro1 = &argumentos[1];
    // let parametro2 = &argumentos[2];

    // println!("El primer parámetro es: {}", parametro1);
    // println!("El segundo parámetro es: {}", parametro2);


    let argumentos: Vec<String> = env::args().collect();

    if argumentos.len() > 2 {
        let parametro1 = &argumentos[1];
        let parametro2 = &argumentos[2];

        println!("El primer parámetro es: {}", parametro1);
        println!("El segundo parámetro es: {}", parametro2);

    }
    else {
        println!("Formato incorrecto");
    }

}
