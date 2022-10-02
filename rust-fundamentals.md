## Instalando Rust (MacOS o Linux)

En el siguiente link se encuentra la documentación oficial para la instalación de Rust: [Install Rust - Official](https://www.rust-lang.org/tools/install)

Basicamente con el siguiente comando lanzandolo en tu terminal instalas Rust en tu equipo:
- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Eligiendo la opción por default es suficiente. 

Para corroborar la instalación, primero tienes que reiniciar tu actual terminal de comandos o ejecutar el comando `source $HOME/.cargo/env` (se te mostrará en la terminal de cualquier manera) para preparar la actual. Luego, ejecuta el comando `cargo` que es el manejador de packetes de Rust y visualizarás el lenguaje correctamente instalado en tu ordenador.

> Easy-Peasy

¿Usas Visual Studio Code? --> Instala la extensión oficial de Rust en VS Code: [Rust for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

## Hello World Rust


En Rust, con utilizar el comando `cargo new <YOUR_PROJECT_NAME>`, el CLI de Rust creará los siguientes archivos automaticamente:

[Hello, World! Rust](hello-world/)

- `src`: Directorio principal con todo el código fuente del proyecto.
- `main.rs`: Archivo principal para lanzar el proyecto.
- `.gitignore`: Archivo para indicarle a GIT qué archivos/directorios ignorar.
- `Cargo.toml`: Archivo que contiene la metadata y dependencias del proyecto.

El archivo `src/main.rs` contiene el *Hello, World!* generado automaticamente para ejecutar tu primer proyecto en Rust.

```Rust
fn main() {
    println!("Hello, world!");
}
```

Estando en la carpeta del proyecto y con el comando `cargo run`, deberías visualizar Hello, world! en la terminal de comandos.

> Ok, end of Hello, World! Rust... now we can start with the fundamentals...
