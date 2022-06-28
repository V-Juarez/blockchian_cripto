fn main() {
    // Obtener el nobmre del usuario
    println!("Por Favor ingresa tu nombre: ");
    let mut nombre : String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    // Obtener el pais del usuario
    println!("Por favor introduce tu pais: ");
    let mut pais : String = String::new();
    std::io::stdin().read_line(&mut pais).unwrap();
    pais = pais.trim().to_string();

    println!("Hola, bienvenido o bienvenida {} de {}", nombre, pais);
}
