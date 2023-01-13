fn main() {
    println!("Esta es tu ultima oportunidad. Despues de esto, no hay vuelta atras. \
        Tomas la pildora azul: la historia termina, te despiertas en tu cama y crees \
        lo que quieras creer. Tomas la pildora roja: te quedas en el Pais de las Maravillas \
        y te ensennp la profunda que es la madriguera del conejo. Reucerda: todo lo que \
        ofrezco es la verdad. Nada mas");
    println!("Que pastilla tomaras, roja o azul: ");
    let mut pildora : String = String::new();
    std::io::stdin().read_line(&mut pildora).unwrap();

    // Limpist entrada de pildora,
    pildora = pildora.trim().to_string();

    // Si es rojo, nos quedamos en la matrix.
    // Se es azul, salimos de la matrix
    // Cualquier otro caso: La matrix explota y mueren todos
    if pildora == "roja" {
        println!("Sales de la matrix y ves hata donde llega la madriguera del conejo");
    } else if pildora == "azul" {
        println!("Vuelves a la matrix y no paso nada...");
    } else {
        println!("La matrix explota y todos mueren...");
    }
    
}
