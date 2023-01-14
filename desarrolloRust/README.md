## Cómo hacer “Hola mundo” con WASM

Asegúrate de tener instalado en tu ordenador Rust. Tomate un rato para hacerlo y apóyate del [Curso Básico de Rust](https://platzi.com/cursos/rust-basico/).

### Paso 1: preparación del entorno

De forma muy sencilla, puedes instalar WASM para Rust con el comando `cargo install wasm-pack`. Tardará unos pocos minutos en estar listo.

[wasm-pack | Rust](https://rustwasm.github.io/wasm-pack/installer/#)

### Paso 2: setup del proyecto

Crea un nuevo proyecto de Rust con el comando `cargo new --lib <project-name>`- Observa el `--lib` para crear un nuevo proyecto en formato de librería.

**ejecutar**

```sh
cd src/
cargo build
```

El comando creará `src/lib.rs` que es archivo raíz de nuestra librería. A diferencia de los proyectos normales de Rust donde el archivo se llama `main.rs`.

A continuación, prepara el `Cargo.yml` de tu proyecto con las dependencias.

```json
[package]
name = "rust-wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

La sección `[lib]` agrega la configuración necesaria para que Rust reconozca qué tipo de librería estamos desarrollando.

> NOTA: Cargo posee [múltiples tags](https://doc.rust-lang.org/cargo/reference/cargo-targets.html) para el archivo `Cargo.yml` que puedes explorar sus utilizades y tipos de configuraciones y dependencias.

### Paso 3: desarrollo de la librería

En el archivo `lib.rs`, puedes desarrollar tus primeras funciones con Rust y WASM de la siguiente manera:

```rust
use wasm_bindgen::prelude::*;

// Indicamos a Rust que utilizaremos la función `alert()` de Javascript.
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// Función expuesta que utilizaremos desde el navegador
#[wasm_bindgen]
pub fn saludar(nombre: &str) {
    alert(&format!("Hola, {}, ¿como estas?", nombre))
}
```

### Paso 4: compilación de la librería

Para compilar nuestra librería, has uso del comando `wasm-pack build --target web`. El mismo creará un directorio llamado `pkg` contiene la librería de WASM desarrollada con Rust.

El archivo `<project_name>.js` es el que enviaremos al navegador para utilizarlo junto con Javascript.

### Paso 5: front-end del proyecto

Creemos un simple HTML con Javascript que se conectará con WASM. Crea un archivo llamado `index.js` en la raíz del proyecto.

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Mi primer proyecto en WASM</title>
  </head>

  <body>
    <script type="module">
      // Importamos el JS creado y el mismo traerá el binario de WASM
      import init, { saludar } from "./pkg/rust_wasm.js";

      // Inicializamos con WASM y ejecuramos la función saludar desarrollada en Rust
      init().then(() => {
        saludar("Kevin");
      });
    </script>
  </body>
</html>
```

### Paso 6: WASM en el navegador web

Teniendo todo listo, deberás levantar un Servidor Web para el `index.js`. Si tienes Python instalado en tu navegador, utiliza el comando `python3 -m http.serve 8080` para levantar uno en cuestión de segundos.

Ingresa a `localhost:8080/` y si todo ha ido bien, visualizarás un `alert()` pero ejecutado desde WASM.

Si inspeccionas la red en el navegador con las herramientas de desarrolladores, observarás dos archivos. Uno `.js` y otro `.wasm`. Entre ambos permiten la conexión entre **Javascript** y **WebAssembly**.

![Screenshot de WASM en el navegador web](https://cdn.document360.io/da52b302-22aa-4a71-9908-ba18e68ffee7/Images/Documentation/Screenshot%20from%202022-08-02%2014-30-26.png)

## Conclusión

Has visto lo sencillo que es desarrollar tu primera aplicación entre Rust y WASM, y conectar la misma con Javascript. Es súper interesante esta combinación de tecnologías.
