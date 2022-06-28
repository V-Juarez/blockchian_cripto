fn main() {
    // let edad: u8 = 85;
    // let name: &str = "Omar";
    // println!("Hello, Soy {} y tengo {} annos", name, edad);
    //
    //
    // let max_temp: i32 = 80;
    // let min_temp: i32 = -40;
    //
    // println!("The max temperature is {}", max_temp);
    // println!("The min temperature is {}", min_temp);

    // Obtener edad por consola
    println!("Igresa tu nombre: ");

    let mut name : String = String::new();

    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("Igresa tu edad: ");

    // Obtener edad por consola
    let mut edad : String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    // convertir edad en numero
    let edad_int : u8 = edad.trim().parse().unwrap();

    println!("Hola, Bienvenid@ {}, al curso de Rust", name);
    println!("awesome, fine {} age", edad_int);

    // condicional dentro de un condicional
    if edad_int >= 18 {
        println!("Puedes ingresar a la discoteca");
            if edad_int > 70 {
                println!("Quizas deberias considerar no entrar a la discoteca")
            } else {
                println!("No puedes ingresar a la discoteca")
            } 
    } else {
        println!("No puedes ingresar")
    }

    // refactorin condicional
    if edad_int >= 18 && edad_int != 30 {
        println!("Puedes entrar a la discotecar")
    } else if edad_int == 30 {
        println!("No admitioms personas de exactamente 30 ages")
    }


    if true {
        println!("Esto se va a cumplir siempre");
    } else {
        println!("Esto no se va a complir nunca");
    }


    if false {
        println!("Esto no se va a complir nunca");
    } else {
        println!("Esto se va a cumplir siempre");
    }
}
