<h1>Basic Rust</h1>

<h3>Hector</h3>

<h2>Table of Contents</h2>

- [1. Inicia tu camino con Rust](#1-inicia-tu-camino-con-rust)
  - [Bienvenida al Curso Básico de Rust](#bienvenida-al-curso-básico-de-rust)
  - [Rust: un lenguaje amado por los desarrolladores](#rust-un-lenguaje-amado-por-los-desarrolladores)
  - [Instalando Rust en MacOS o Linux](#instalando-rust-en-macos-o-linux)
  - [Instalando Rust en Windows](#instalando-rust-en-windows)
- [2. Domina las bases de Rust](#2-domina-las-bases-de-rust)
  - [Creando un nuevo proyecto](#creando-un-nuevo-proyecto)
  - [Variables fantásticas y cómo mostrarlas en pantalla](#variables-fantásticas-y-cómo-mostrarlas-en-pantalla)
  - [Recibiendo datos del usuario](#recibiendo-datos-del-usuario)
  - [Condicionales](#condicionales)
  - [Ciclo Loop](#ciclo-loop)
- [3. Primer proyecto: calculadora digital](#3-primer-proyecto-calculadora-digital)
  - [Descripción del proyecto](#descripción-del-proyecto)
  - [Cargo (dependencias)](#cargo-dependencias)
  - [¿Qué significa unwrap?](#qué-significa-unwrap)
  - [Creando nuestra calculadora](#creando-nuestra-calculadora)
- [4. Estructuras y funciones en Rust](#4-estructuras-y-funciones-en-rust)
  - [Arrays y Ciclo For](#arrays-y-ciclo-for)
  - [Las funciones en Rust](#las-funciones-en-rust)
- [5. Segundo proyecto: videojuego de texto](#5-segundo-proyecto-videojuego-de-texto)
  - [Descripción del proyecto: videojuego de texto](#descripción-del-proyecto-videojuego-de-texto)
  - [Creación y descripción del entorno](#creación-y-descripción-del-entorno)
  - [Estructuras básicas, narrativa y opciones](#estructuras-básicas-narrativa-y-opciones)
  - [Interacción con el entorno y datos del usuario](#interacción-con-el-entorno-y-datos-del-usuario)

# 1. Inicia tu camino con Rust

## Bienvenida al Curso Básico de Rust

<img src="https://raw.githubusercontent.com/rochacbruno/rust_memes/master/img/ferris_happy.jpg" alt="Ferris, mascota de Rust!" style="zoom:50%;" />

## Rust: un lenguaje amado por los desarrolladores

[La biblia de Rust](https://doc.rust-lang.org/book/title-page.html)

- [enlace](<https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/> ) por si quieren profundizar un poco mas sobre las capacidades del lenguaje Rust 👾

[![img](https://www.google.com/s2/favicons?domain=https://insights.stackoverflow.com/content/img/survey/2021/favicon.8e42b0a2.ico)Stack Overflow Developer Survey 2021](https://insights.stackoverflow.com/survey/2021)

## Instalando Rust en MacOS o Linux

Estando en bash si pueden usar ese OneLiner

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

[![img](https://www.google.com/s2/favicons?domain=https://www.rust-lang.org/tools/install/static/images/apple-touch-icon.png?v=ngJW8jGAmR)Install Rust - Rust Programming Language](https://www.rust-lang.org/tools/install)

[![img](https://www.google.com/s2/favicons?domain=https://code.visualstudio.com//favicon.ico)Visual Studio Code - Code Editing. Redefined](https://code.visualstudio.com/)

[![img](https://www.google.com/s2/favicons?domain=https://static.platzi.com/media/favicons/platzi_favicon.png)Rust - Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

## Instalando Rust en Windows

Instalar Rust en Windows es tremendamente fácil, tenemos dos opciones:

- Instalarlo en Windows
- Instalarlo en WSL

Para la primera opción, todo lo que tenemos que hacer es lo siguiente:

1. Debemos tener instalado [Microsoft C++ Build tools for visual studio 2013](https://visualstudio.microsoft.com/visual-cpp-build-tools/) o superior.
2. Ir a la [web de Rust](https://www.rust-lang.org/tools/install).
3. Darle clic al instalador que necesitemos, en mi caso de 64 bits.
4. Esto abrirá una interfaz muy sencilla que instalará Rust.
5. Ya podemos seguir el curso sin problemas. ✅

Para la siguiente opción es aún más fácil todavía, es muy parecido a lo que vimos en la clase anterior:

1. Asegurarse de [tener instalado WSL](https://docs.microsoft.com/en-us/windows/wsl/install).
2. Ejecutar en una consola WSL el siguiente comando:

```js
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs/) | sh
```

Cualquiera de las dos partes nos ayudará a continuar con el curso, la diferencia es únicamente nuestra preferencia personal, yo elegiría WSL. Sin embargo, es únicamente mi opinión.

# 2. Domina las bases de Rust

## Creando un nuevo proyecto

Crear proyecto

```sh
cargo new platzi-rust
# cargo new <name-project>
```

`hello.rs`

```rust
fn main() {
    let edad: u8 = 85;
    let name: &str = "Omar";
    println!("Hello, Soy {} y tengo {} annos", name, edad);
}
```

ejecutar `rust` 

```sh
# ejecutar desde el directorio principal
cargo run
```

Al manejar un número definido de bits, cada variable puede albergar  hasta un cierto número de valor (Por ejemplo, si tratáramos de guardar  un 256 en una variable de tipo `u8`, me saltaría error de `Out of range (Fuera de rango)`

![img](https://miro.medium.com/max/354/1*5bd_OBT6Nha2Wl05asyqfg.png) 

La diferencia entre los **signed** y **unsigned**, es que estos últimos solo utilizan su capacidad para almacenar números positivos, mientras que los **signed** lo usan para una cantidad igual de números positivos y negativos.

![img](https://miro.medium.com/max/1400/1*MxVEixCs1iS1shQs2JVTYg.png)

![img](https://miro.medium.com/max/1400/1*ScXl3GI_8EY0Ow4t-1dRUg.png)

`temperatura.rs`

```rust
fn main() {
    let max_temp: i32 = 80;
    let min_temp: i32 = -40;

    println!("The max temperature is {}", max_temp);
    println!("The min temperature is {}", min_temp);
}
```

## Variables fantásticas y cómo mostrarlas en pantalla

Pequeño dato, Rust tiene como estandar usar snake case para declaraciones, en  caso de que no la utilices te lo señalizara como un warning en tiempo de compilacion.

```rust
fn main() {
// snake case work fine
 let var_en_snake_case: u8 = 8 ;
 
 //Camel Case, Warning in compilation time
 let varEnCamelCase: u8 = 8;
}
```

desestructurar las librerias importanto solo el componente que se va a utilizar con la palabra reservada use

```rust
use std::io;

fn main() {
    println!(
        "Hello {}, you have {} years old!",
        &get_name_from_user(),
        &get_age_from_user()
    );
}

fn get_name_from_user() -> String {
    println!("Please insert your name");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    name.trim().to_string()
}

fn get_age_from_user() -> u8 {
    println!("Please insert your age");

    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    age.trim().parse().unwrap()
}
```

## Recibiendo datos del usuario

datos por consola

```rust
fn main() {  
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

}

```

formularion reto

```rust
fn main() {
    println!("Please! Enter your name: ");
    let mut name : String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("Please! Enter your age: ");
    let mut age : String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    let age_int : u8 = age.trim().parse().unwrap();

    println!("Please! Enter your country of origin: ");
    let mut country : String = String::new();
    std::io::stdin().read_line(&mut country).unwrap();
    country = country.trim().to_string();

    println!("Hello Welcome {} you age is {} years old and your country origin is {}", name, age_int, country);
}
```

## Condicionales

asignaciones de variables con condicionales ternarios

```rust
let variable = if true { "Enabled" } else { "Disabled" };
```

from

```rust
fn main() {
    println!("Esta es tu última oportunidad. Después, ya no podrás echarte atrás. Si tomas la pastilla azul, fin de la historia. Despertarás en tu cama y creerás lo que quieras creerte. Si tomas la roja, te quedas en el País de las Maravillas y yo te enseñaré hasta dónde llega la madriguera de conejos. Recuerda lo único que te ofrezco es la verdad. Nada más.");
    println!("Qué pastilla tomarás? roja o azul?");

    let mut opcion: String = String::new();
    std::io::stdin().read_line(&mut opcion).unwrap();
    let pastilla: &str = opcion.trim();

    if pastilla == "roja" {
        println!("Muy bien, Neo. Sígueme....");
    } else if pastilla == "azul" {
        println!("Como prefieras. No nos veremos nunca mas....");
    } else {
        println!("Tal parece que no eres el Neo que pensábamos.");
    }
}

```

pildora roja o azul

```rust
use std::io;

fn main() {
    let pildora_roja: &str = "🔴";
    let pildora_azul: &str = "🔵";
    let mut opcion: String = String::new();

    println!("¿Cual pildora deseas tomar?");
    println!("1 - Pildora roja {}", pildora_roja);
    println!("2 - Pildora azul {}", pildora_azul);

    io::stdin().read_line(&mut opcion).unwrap();

    let op_int: u8 = opcion.trim().parse().unwrap();

    if op_int == 1 {
        println!("Elegiste la pildora {}", pildora_roja)
    } else if op_int == 2 {
        println!("Elegiste la pildora {}", pildora_azul)
    }else{
        println!("Opción invalida")
    }
}
```



## Ciclo Loop

# 3. Primer proyecto: calculadora digital

## Descripción del proyecto

## Cargo (dependencias)

## ¿Qué significa unwrap?

## Creando nuestra calculadora

# 4. Estructuras y funciones en Rust

## Arrays y Ciclo For

## Las funciones en Rust

# 5. Segundo proyecto: videojuego de texto

## Descripción del proyecto: videojuego de texto

## Creación y descripción del entorno

## Estructuras básicas, narrativa y opciones

## Interacción con el entorno y datos del usuario

