# Fundamentos de Rust

- [Fundamentos de Rust](#fundamentos-de-rust)
  - [Instalando Rust (MacOS o Linux)](#instalando-rust-macos-o-linux)
  - [Hello World Rust](#hello-world-rust)
  - [Variables de Rust y cómo mostrarlas en pantalla](#variables-de-rust-y-cómo-mostrarlas-en-pantalla)
    - [Declaración de variables en Rust](#declaración-de-variables-en-rust)
    - [Variables numericas en Rust](#variables-numericas-en-rust)
    - [Variables del tipo string en Rust](#variables-del-tipo-string-en-rust)
    - [println!() - Mostrar variables y/o mensajes en pantalla](#println---mostrar-variables-yo-mensajes-en-pantalla)
    - [Ejemplo](#ejemplo)
  - [Recibiendo datos del usuario](#recibiendo-datos-del-usuario)
    - [Cambiando el tipo de dato en Rust](#cambiando-el-tipo-de-dato-en-rust)
  - [Condicionales](#condicionales)

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

## Variables de Rust y cómo mostrarlas en pantalla

Como todo lenguaje de programación, Rust posee diversos tipos de variables para almacenar números, cadenas de caracteres, etc.

### Declaración de variables en Rust

Rust es un lenguaje **fuertemente tipado**, lo que significa que tienes que hacer explícito el tipo de las variables y este no pueden cambiar en tiempo de ejecución.

Como las variables en rust son **inmutables** (no pueden modificarse) tanto en su valor como es su tipo, para hacer una variable mutable se debe agregar la palabra reservada **mut** y el tipo de dato se debe mantener.

```Rust
    let [mut (if we want a mutable variable)] <VARIABLE_NAME>: <DATA_TYPE> = <VALUE> 
```

### Variables numericas en Rust

- Enteros con signo: `i8`, `i16`, `i32`, `i64`, `i128` and `isize` (pointer size)
- Enteros sin signo: `u8`, `u16`, `u32`, `u64`, `u128` and `usize` (pointer size)
- Decimales: `f32`, `f64`

### Variables del tipo string en Rust

Para el guardado de variables del tipo cadenas de texto, el tipado se realiza con la palabra reservada `&str`.

### println!() - Mostrar variables y/o mensajes en pantalla

Para utilizar y mostrar por pantalla dichas variables, utiliza el comando `println!()` e ingresa el texto que deseas mostrar.

Las variables que le pases después de la primera coma, reemplazarán los `{}`, denominados “placeholders”, y formateará el texto en el orden que hayas establecido el ingreso de cada valor.

### Ejemplo

```Rust
fn main() {
    let age: i16 = 24;
    let name: &str = "Peter Parker";
    let mut year: u16 = 2022;

    year += 1;

    println!("Hello, world!");
    println!("My age is {} and my name is {}", age, name);
    println!("Next year will be {}", year);
}

```

Podemos correr nuestro proyecto de Rust con el comando `cargo run` en la terminal estando en la carpeta del archivo [main.rs](hello-world/src/main.rs), para este caso, y visualizar los mensajes y los valores de las variables desde la terminal de línea de comandos.

## Recibiendo datos del usuario

Recibir datos de un usuario con tu aplicación es una de las bases de cualquier lenguaje de programación. Podemos hacer esto con Rust desde la interfaz de línea de comandos.

```Rust
fn main() {
    println!("Ingrese su nombre:");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    println!("Bienvenido o bienvenida: {}", nombre);
}
```

- `println!()`: Solo muestran mensaje en pantalla.
- `let mut nombre: String = String::new();`: Declaramos una variable del tipo string donde almacenaremos el nombre. 
  - Podemos declarar un string con `String` o con `&str`. Las diferencias son mínimas:
    - `String`: Te permite manipular el texto, hacer substrings o splits, pero ocupa algo más de espacio en memoria. Es un objeto en POO.
    - `&str`: Es texto plano sin otra funcionalidad. Hace referencia al contenido de un tipo de dato nativo y el contenido de una posición de memoria.

- `std::io::stdin().read_line(&mut nombre).unwrap();`:
  - `std` es una librería de Rust para acceder al sistema operativo.
  - `io` significa inputs/outputs, `println!()` lo utiliza internamente para mostrar mensajes por consola, aquí lo utilizaremos para ingresar datos por la misma.
  - `stdin()` permite traer información.
  - `read_line()` indica que esa información será recibida por consola. `&mut nombre` es la variable donde guardaremos el dato ingresado por el usuario (Utiliza mut en las variables para indicar que la misma cambiará de valor).
  - `unwrap()`, nos ayuda con el manejo de errores.

- `nombre = nombre.trim().to_string();`: Aquí solo formateamos el texto para eliminar saltos de línea y espacios en blanco.

### Cambiando el tipo de dato en Rust

Por defecto, todos los datos que ingresa el usuario por consola son del tipo `String`. Puede ocurrir que necesites números enteros y para esto tienes que convertir el tipo de datos de la siguiente manera:

```Rust
fn main() {
    println!("Ingrese su edad:");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    let edad_int: u8 = edad.trim().parse().unwrap();

    println!("La edad es: {}", edad_int);
}
```

- `let edad_int: u8 = edad.trim().parse().unwrap();` donde creamos una nueva variable donde guardaremos un número del tipo `u8`. De esta forma podrás manipular ese dato y realizar cualquier operación matemática.

## Condicionales

Ejemplos en: [Conditionals - rust-fundamentals/src/main.rs](rust-fundamentals/src/main.rs)

La sintaxis de un `if/else` en Rust es:

```Rust
fn main() {
    let edad: i8 = 20;
    if edad >= 18 {
        println!("Eres mayor de edad");
    } else {
        println!("Eres menor de edad");
    }
}
```

La condición de un `if` puede ser de varios tipos:

- Validar la igualdad de datos con `==`
- verificar si un número es mayor, menor o igual con `>`, `<`, `>=` y `<=`.
- Agrupar condiciones con `&&` para un AND lógico o un `||` para un OR lógico.
