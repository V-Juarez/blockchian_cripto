fn sumar_uno(numero_a_sumar: i32) -> i32 {
    let numero_final = numero_a_sumar + 1;
    println!("{}", numero_final);

    return numero_final;
}

fn main() {

    let nueve_mas_uno = sumar_uno(9);
    sumar_uno(nueve_mas_uno);
    sumar_uno(11);
    sumar_uno(12);
    sumar_uno(13);
    
    // std::io::stdin().read_line(&mut nombre).unwrap();
}
