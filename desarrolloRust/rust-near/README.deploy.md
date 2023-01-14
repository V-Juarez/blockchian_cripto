En esta ocasión vamos a hacer deploy de nuestro mini contrato inteligente en la blockchain de NEAR, veremos que es tremendamente sencillo, pero a la vez tenemos que tener un poco de cuidado.

1. Lo primero que necesitamos es una wallet de NEAR, podemos crearla [desde aquí](https://wallet.testnet.near.org/), este paso es trivial, pero recuerda guardar con mucho cuidado los detalles de tu wallet, que no se te pierda y que nadie tenga acceso a ella. Vamos a reservarla, ya que la usaremos luego.
2. Vamos a instalar el NEAR CLI, para esto simplemente ejecutamos en nuestra consola este comando:
   1. `npm install -g near-cli`.
   2. Necesitamos tener instalado Node y npm para este propósito.
3. Haremos Login con nuestra wallet en el CLI:
   1. `near login`.
   2. Tus credenciales serán almacenados en la carpeta **`~/.near-credentials`**.
4. Ahora necesitamos compilar nuestro código, para ello utilizaremos el siguiente comando:
   1. `cargo build --target wasm32-unknown-unknown --release`.
5. Esto generará un .wasm listo para ser deployado, se encuentra en la carpeta `target`.
6. Ahora, para hacer deploy a nuestro wasm, simplemente ejecutaremos el siguiente comando (cambiando los valores por nuestros datos).
   1. `near deploy --wasmFile target/wasm32-unknown-unknown/release/<NAME_OF_YOUR_PROYECT>.wasm --accountId < YOUR_ACCOUNT_HERE >`.
7. Este contrato ahora está en la testnet, si quieres cambiar la red, puedes ejecutar este comando:
   1. `export NEAR_ENV=< RED_TO_BE_USED >`.
   2. Por ejemplo, para la main net: `export NEAR_ENV=mainnet`.
8. Para ejecutar los comandos de nuestro contrato, este es el comando:
   1. `near call < YOUR_ACCOUNT_HERE > < NAME_OF_FUNCTION > --accountId < YOUR_ACCOUNT_HERE >`.
   2. `< NAME_OF_FUNCTION >` representa la función que quieres ejecutar, en nuestro ejemplo son increment, decrement, reset, get_num, etc.

Con esto nuestro contrato efectivamente ha sido deployado y hemos obtenido algunos resultados interesantes, nos vemos en la próxima clase en donde hablaremos sobre otra tecnología.