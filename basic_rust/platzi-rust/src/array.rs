fn main() {
    
    let mut nombres : Vec<String> = Vec::new();

    // for i in 0..3 {
    //     println!("Ingresa tu nombre: ");
    //     let mut nombre = String::new();
    //     std::io::stdin().read_line(&mut nombre).unwrap();
    //
    //     nombres.push(nombre);
    //     // println!("{:?}", nombres);
    // }
    //
    for nombre in nombres {
        println!("El nombre es: {}", nombre);
    }
    
    let hola = ["h", "o", "l", "a"];
    println!("{}", hola[0]);
    println!("{}", hola[1]);
    println!("{}", hola[2]);
    println!("{}", hola[3]);
    
}
