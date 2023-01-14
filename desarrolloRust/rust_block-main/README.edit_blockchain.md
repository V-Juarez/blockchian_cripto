Cada modulo dentro de nuestra blockchain debe tener la misma estructura:

```rust
// runtime/src/platzimodule.rs
use support::{decl_storage, decl_module};

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as PlatziStorage {
        // Se declaran aqui los datos del modulo y funciones read-only
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Se declaran aqui las funciones especiales que cambian el estado
    }
}
```

Para agregar nuestro nuevo módulo al runtime, o a las partes que cargan, debemos añadir las siguientes líneas al archivo `lib.rs`

```rust
// runtime/src/lib.rs
...
pub type BlockNumber = u64;
pub type Nonce = u64;

...

mod platzimodule; //<- Añadir el modulo

...

impl sudo::Trait for Runtime {

	...

}

...

impl platzimodule::Trait for Runtime {} //<- Añadir las funciones del modulo

...

construct_runtime!(
    pub enum Runtime with Log(InternalLog: DigestItem<Hash, Ed25519AuthorityId>) where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
		    {
		        System: system::{default, Log(ChangesTrieRoot)},
		        ...
		        // Añadir esta linea
						// Notar que agregamos Module, Call y Storage, 
						// que son los macros antes definidos
		        PlatziModule: platzimodule::{Module, Call, Storage},
		    }
);

...
```

Si hicimos todo correctamente, en este punto podremos compilar sin errores:

```
./scripts/build.sh
cargo build --release
```

Dentro de nuestro Storage, podemos agregar los datos que necesitemos para consulta:

```rust
decl_storage! {
    trait Store for Module<T: Trait> as PlatziStorage {
        CursosVistos: u32;
        EsExpert get(expert_getter): bool;
    }
}
```

También podemos agregar las funciones que necesitemos, como añadir nuevos datos:

```rust
use support::{dispatch::Result, StorageValue};
use system::ensure_signed;

...

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn comprar_expert(origin, input_bool: bool) -> Result {
						// Notar que Origin, habla de la wallet que firma la transaccion.
            let _sender = ensure_signed(origin)?;

            <EsExpert<T>>::put(input_bool);

            Ok(())
        }
    }
}
```

Ya hemos visto lo extensible que es nuestro nodo de substrate, podemos extender este modelo para todo lo que quisiéramos solo con el Storage y los Modules. Si quisiéramos interactuar con nuestro nodo, la forma más fácil es hacerlo desde `substrate-ui` o de manera más avanzada desde `pokadot-js`.