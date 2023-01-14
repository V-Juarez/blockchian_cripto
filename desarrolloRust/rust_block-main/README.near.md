## Cómo trabajar Blockchain local con Substrate

Correr tu **primera Blockchain de forma local** es bastante sencillo. Bastará con clonar los proyectos adecuados y ponerlos a funcionar.

### Paso 1: clonar el backend

Comienza clonando el [repositorio de Substrate](https://github.com/substrate-developer-hub/substrate-node-template), un código base ya funcional que te permitirá desarrollar sobre él tu proyecto.

Seguido a eso, tendrás que preparar el entorno de desarrollo de tu computador con Rust con algunas configuraciones extras con los comandos `rustup update nightly` y `rustup target add wasm32-unknown-unknown --toolchain nightly`.

Es momento de compilar el proyecto con `cargo build --release` y finalmente levantarlo con `./target/release/node-template --dev`.

En este punto, ya tendrás tu Blockchain corriendo en tu computador.

### Paso 2: clonar el front-end

Continúa clonando el [repositorio del front-end](https://github.com/substrate-developer-hub/substrate-front-end-template) de Substrate que te permitirá visualizar la información de la Blockchain de prueba que hemos levantado en forma local.

El mismo se encuentra desarrollado con React y necesitarás *Yarn* para instalar las dependencias del proyecto con el comando `yarn install`.

Ya tienes todo listo para levantar el front-end con el comando `yarn start` que expondrá el proyecto en el puerto 8000 de tu computador para visualizar las transacciones de la Blockchain.

## Conclusión sobre Substrate

Con estos proyectos, eres capaz de comenzar a explorar el desarrollo tanto de una Blockchain con Rust y Substrate como de un front-end para visualizar los datos de una Blockchain. A partir de tus intereses, puedes enfocarte y profundizar en el backend, en el front-end o en ambos proyectos.

Muchísimo por aprender si realmente es de tu interés el desarrollo Blockchain/Web3 en este ecosistema, que es una gran alternativa al ecosistema de Ethereum que toma fuerzas con el paso del tiempo.