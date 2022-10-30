
- [Introduccion](#introduccion)
  - [Instalando Rust (MacOS o Linux)](#instalando-rust-macos-o-linux)
  - [Hello World Rust](#hello-world-rust)
- [Bases de Rust](#bases-de-rust)
  - [Variables de Rust y cómo mostrarlas en pantalla](#variables-de-rust-y-cómo-mostrarlas-en-pantalla)
    - [Declaración de variables en Rust](#declaración-de-variables-en-rust)
    - [Variables numericas en Rust](#variables-numericas-en-rust)
    - [Variables del tipo string en Rust](#variables-del-tipo-string-en-rust)
    - [println!() - Mostrar variables y/o mensajes en pantalla](#println---mostrar-variables-yo-mensajes-en-pantalla)
    - [Ejemplo](#ejemplo)
  - [Recibiendo datos del usuario](#recibiendo-datos-del-usuario)
    - [Cambiando el tipo de dato en Rust](#cambiando-el-tipo-de-dato-en-rust)
  - [Condicionales](#condicionales)
  - [Ciclo Loop](#ciclo-loop)
- [Primer proyecto: Calculadora cientifica](#primer-proyecto-calculadora-cientifica)
  - [Cargo, el geston de dependencias de Rust](#cargo-el-geston-de-dependencias-de-rust)
    - [Instalación de una dependencia](#instalación-de-una-dependencia)
    - [Utilización de una dependencia](#utilización-de-una-dependencia)
    - [Consejos sobre utilización de dependencias de terceros](#consejos-sobre-utilización-de-dependencias-de-terceros)
  - [Manejo de errores en Rust - unwrap()](#manejo-de-errores-en-rust---unwrap)
  - [Realizando la calculadora cientifica](#realizando-la-calculadora-cientifica)
- [Estructuras y funciones en Rust](#estructuras-y-funciones-en-rust)
  - [Arrays de datos](#arrays-de-datos)
  - [Ciclo for](#ciclo-for)
  - [Funciones](#funciones)
    - [Declarando y llamando a una función](#declarando-y-llamando-a-una-función)
    - [Retorno de valores de una funcion](#retorno-de-valores-de-una-funcion)
- [Segundo proyecto: videojuego de texto](#segundo-proyecto-videojuego-de-texto)
  - [Leer archivos](#leer-archivos)
  - [Valores por defecto en los errores](#valores-por-defecto-en-los-errores)
  - [Estructura de datos e implementación de las mismas](#estructura-de-datos-e-implementación-de-las-mismas)
  - [Almacenamiento clave/valor o **HashMap**](#almacenamiento-clavevalor-o-hashmap)
  - [Accediendo al índice de un vector](#accediendo-al-índice-de-un-vector)
  - [Option, Some y None](#option-some-y-none)

# Introduccion

- Intalacion de Rust
- Hello World! Rust...

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

# Bases de Rust

Estas son las herramientas básicas de cualquier lenguaje de programación para desarrollar software:

- Declaración de variables y sus tipos
- Captura de inputs del usuario
- Condicionantes
- Ciclos iterativos

Con estos conceptos tendremos lo esencial para programar aplicaciones básicas en Rust.

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

## Ciclo Loop

Ejemplos en: [ciclo loop - rust-fundamentals/src/main.rs](rust-fundamentals/src/main.rs)

Los ciclos iterativos en Rust realmente rompen con el paradigma de sintaxis de programación en comparación con otros lenguajes.

```Rust

fn main() {
    loop {
        println!("Ingrese 123 para detener el loop:");
        let mut number: String = String::new();
        std::io::stdin().read_line(&mut number).unwrap();
        let number_int: i32 = number.trim().parse().unwrap();

        if number_int == 123 {
            break;
        }
    }
}
```

La palabra reservada `loop` crear un ciclo de iteraciones del código fuente en su interior. El ciclo se repetirá hasta encontrar un `break`.

`loop` es una manera diferente de crear un ciclo iterativo. Es crucial tener cuidado en nuestros algoritmos y que sea bien probado para no entrar en bucles infinitos, y así asegurar que el mismo tenga un punto de finalización.

# Primer proyecto: Calculadora cientifica

- Cargo y manejo de paquetes.
- Manejo de errores en Rust - unwrap()
- Operaciones matemáticas básicas de Rust.
- Algo de REGEX...

## Cargo, el geston de dependencias de Rust

Todo lenguaje de programación tiene su gestor de dependencias. `Composer` para PHP, `NPM` o `Yarn` para Javascript, `Pip` para Python, `Maven` para Java, entre otros. En Rust utilizamos `Cargo`.

`Cargo` posee una importante cantidad de dependencias desarrolladas por la comunidad del lenguaje. Puedes encontrar y buscar las dependencias que necesitas en [Crates.io](https://crates.io/) que es el repositorio de dependencias de Rust que Cargo usa para descargarlas.

### Instalación de una dependencia

Para instalar una dependencia basta con agregar manualmente el nombre seguido de la versión de la misma al archivo `Cargo.toml` debajo de la sección `[dependencies]`.

```Rust
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
regex = "1.6.0"
```

La próxima vez que realices un `cargo run`, este detectará que la dependencia no se encuentra instalada y realizará la instalación de la misma.
Realiza la acción contraria de borrar la dependencia manualmente para que Cargo lo detecte y borre la misma de tu proyecto.

### Utilización de una dependencia

Para utilizar una dependencia en tu proyecto, realiza la importación de la siguiente manera:

```Rust
use regex::Regex;

fn main() {
    // ...
}
```

Cargo genera un nuevo archivo en la raíz de tu proyecto llamado `Cargo.lock`. El mismo contiene las versiones exactas de las librerías de nuestro proyecto. Es importante versionar en Git este archivo para que todos utilicen las mismas dependencias cuando utilicen el proyecto.

### Consejos sobre utilización de dependencias de terceros

- Las buenas dependencias desarrolladas por la comunidad de Rust suelen tener una documentación de uso, además de estar actualizadas.
- Antes de descargar cualquier dependencia, chequea quién la desarrolla, qué cantidad de descargar por semana posee, revisar cuándo recibió su última actualización en el repositorio oficial de la misma (normalmente en GitHub).
- Es importante seleccionar buenas dependencias sin bugs y que no generen problemas de seguridad en tu aplicación.
- Cargo es un poderoso gestor de dependencias, pero es más que la instalación de librerías de terceros que necesites. Tiene múltiples usos, podemos explorarlo con el comando `cargo --help` para visualizar por consola todas sus posibilidades.

## Manejo de errores en Rust - unwrap()

- En Rust, **no existen los valores nulos**. Rust busca la seguridad y la robustez en el software con esto no permite los errores en tiempo de ejecución producto del valor nulo de una variable.

- En Rust **todo es un tipo de dato**. El tipo de dato [Option](https://doc.rust-lang.org/std/option/), contiene dentro el tipo de dato que puede existir o no como un número entero o un string. Devolverá el mismo, si existe, o nos devolverá en caso contrario un [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None) que indica su no existencia, pero que no es lo mismo que `null`, ya que es un tipo de dato, y `null` un espacio en memoria totalmente vacio.

- Una operación en Rust devuelve el tipo de dato [Result](https://doc.rust-lang.org/std/result/), donde:
  - T es el valor de la operación exitosa
  - E un posible error.

- En Rust, los errores también son un tipo de dato. Para manejarlos, utilizamos .unwrap(). el mismo nos devuelve el valor T que necesitamos si este existe o E en caso de producirse un error.

> Nota: En Rust no existe la sentencia `try/catch` para la captura de errores como en otros lenguajes.

Si no implementamos `.unwrap()`, podemos obtener un **warning** por consola que nos advierte de la posibilidad del error en una operación. De esta manera, gracias a este tipo de dato que contiene el error, podemos actuar en consecuencia a partir de identificar el tipo de error. Similar a un traceback en Python.

## Realizando la calculadora cientifica

[Proyecto de Calculadora científica](digital-calculator/src/main.rs)

- La calculadora la hice con los conocimientos basicos que tengo sobre Rust. Se puede utilizar otras funcionalidades, pero en este punto de mi aprendizaje no me se mas opciones sobre el lenguaje.
- Se utilizo funciones para aislar en division, multiplicacion, suma y resta.
- Se pueden agregar mas operaciones como exponencial, agregar parentesis, etc.
- Para el caso anterior se deberá trabajar en expresiones regulares para realizar esas acciones.
- Se puede modularizar mas el codigo ya que hay partes que se repiten y no son buena práctica.

# Estructuras y funciones en Rust

- Arrays
- Ciclo For
- Funciones

## Arrays de datos

Un array de datos en en Rust es denominado Vector, para declararlo vacio y utilizarlo lo podemos hacer de la siguiente manera:

```Rust
    // Declaro array
    let mut my_arr: Vec<String> = Vec::new();
    
    // Ingreso valoresa al array
    my_arr.push("value_1");
    my_arr.push("value_2");
    my_arr.push("value_3");

    println!{"{:?}", my_arr}
```

- Utilizo la palabra reservada `Vec` para tipar un array.
- Declaro el tipo de dato del array con: `Vec<data_type>`.
- Con el método `.push()` ingreso valores al array.
- Con `mut my_array` lo puedo hacer muteble.
- Puedo usar el metodo `.len()` para saber cuantos valores tiene mi array.
- Uso `"{:?}"` en un `println!()` para imprimir mi array.
- Accedo a los elementos del array como en python: `my_arr[num_element]`

## Ciclo for

Ciclo `for` en un rango:

```Rust
for i in 0..5{
  // Muestro los valores de 0 a 5
  println!{"{}", i}
}
```

Ciclo `for` a traves de la cantidad de valores de un array:

```Rust
for i in 0..my_arr.len(){
  // Muestro los valores de 0 a la cantidad de valores que haya en el array
  println!{"{}", i}
}
```

Ciclo `for` mostrando los elementos de mi array:

```Rust
for value in my_arr{
  // Muestro los valores del array
  println!{"{}", value}
}
```

## Funciones

- Una función es **un bloque de código que realiza alguna operación**.
- Una función puede definir opcionalmente parámetros de entrada que permiten a los llamadores pasar argumentos a la función.
- Una función también puede devolver un valor como salida.

### Declarando y llamando a una función

- La declaración de funciones en Rust se realiza con la palabra reservada `fn`. 
- La función `fn main() {}` es la principal de una aplicación y se ejecuta siempre al iniciar la misma.
- Las funciones puede recibir **argumentos**. Debemos especificar el tipo de los argumentos.

```Rust
fn main() {
    sumar_numeros(10, 10);          // 20
}

fn sumar_numeros(num1: i32, num2: i32) {
    let sum: i32 = num1 + num2;
    println!("{}", sum);
}
```

### Retorno de valores de una funcion

- Entre parentesis de la funcion definimos **los argumentos**.
- Seguido de lo anterior con `->` especificamos el tipo de valor a retornar.
- Utilizamos la palabra reservada `return` de devolver del valor.

```Rust
fn sumar_numeros(num1: i32, num2: i32) -> i32 {
    let sum: i32 = num1 + num2;
    return sum;
}
```

# Segundo proyecto: videojuego de texto

- Leer archivos en Rust (.csv, .json, ...)
- Valores por defecto ante un error.
- Estructura de datos e implementacion de las mismas
- Almacenamiento clave/valor o **HashMap**

## Leer archivos

La **manipulación de archivos** es importarte en cualquier lenguaje de programación, como JSON, CSV, TXT, etc. 

Acceder a archivos:

- Para leer archivos en Rust debemos importar `fs` desde `std`.
- `std` permite acceder al sistema operativo y realizar diversas funcionalidades.
- Podemos definir la ubicación del archivo con una constante `const`, el nombre debe ir siempre en **mayusculas**.
- Con `fs::read_to_string` podemos leer un archivo en la raíz del proyecto.
- Siempre es necesario la utilización de `unwrap()` para capturar errores, ya que el archivo podría no existir.

```Rust
use std::{fs};

const FILENAME: &str = "my_file.csv";

fn main() {
    let content: String = fs::read_to_string(FILENAME).unwrap();
    println!("{}", content);
}
```

El archivo leido se lo considera como string usando la libreria `std::fs`, para manipular exclusivamente csv deberemos instalar una dependencia en `Cargo.toml`:

```Rust
[dependencies]
csv = "1.1.6"
```

- De la libreria `csv` usamos `ReaderBuilder`
- `ReaderBuilder` permite leer el contenido capturado previamente con `fs`, indicando el tipo de delimitador y el tipo de dato como binario. Convertimos el `String` en un `Vector` que contiene el contenido de nuestro archivo.
- Recorremos el contenido de nuestro archivo con un bucle for.
- Devuelve el vector de registros con `records()`
- Accedemos a la columna del CSV que necesitemos con `get(1)` en cada iteración.
- Siempre debemos asegurarnos de utilizar `unwrap()` para contener los errores.

```Rust
use csv::{ReaderBuilder};
use std::{fs};
const FILENAME: &str = "my_file.csv";

fn main() {
    let content: String = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());
    for result in rdr.records() {
      println!("{}", result.unwrap().get(0).unwrap().trim());
    }
}
```

Podemos leer otros tipos, solo debemos explorarlos:
- [json](https://crates.io/crates/json)
- [Excel](https://crates.io/crates/simple_excel_writer)

## Valores por defecto en los errores

Para evitar que nuestra aplicación se detenga por un error y que continúe ejecutándose, podemos colocar un valor por defecto utilizando `unwrap_or()`.

```Rust
let mut age: String = String::new();
std::io::stdin().read_line(&mut age).unwrap();
let age_int: u8 = age.trim().parse().unwrap_or(18);
```

Si el usuario ingresa una letra en lugar de un número cuando se le solicita su edad, la conversión del tipo de dato fallará, pero el `unwrap_or()` devolverá un valor establecido por defecto y la aplicación continuará operando.

## Estructura de datos e implementación de las mismas

Las estructuras de datos sirven para almacenar atributos pertenecientes a un misma objeto, todos dentro de una misma variable que los englobe, como si de una clase se tratase.

Con la palabra clave **struct**, declaras las propiedades de tu estructura y puedes crear una variable que almacene estos datos y acceder a ellos mediante un punto “.”.

Puedes crear estructuras que tendrán la forma de tus datos de la siguiente manera:

```Rust
struct Person {
    first_name: String,
    last_name: String,
    age: i32,
    country: String,
}

fn main() {
    let person = Person {
        first_name: "Peter".to_string(),
        last_name: "Parker".to_string(),
        age: 16,
        country: "EEUU".to_string(),
    };

    println!("{}", person.first_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
    println!("{}", person.country);
}
```

> NOTA: Las estructuras utilizan CamelCase para nombrar a las mismas a diferencia de las variables o funciones que utilizan snake_case.

Ademas una estructura se puede extender e implementar funciones para realizar operaciones lógicas.

Utilizamos la palabra reservada `impl`, la función `new_person` creará un objeto `Person` pasándole como parámetro los datos que necesita y devolviendo el mismo para su posterior utilización.
Hacemos el llamado a estas funciones implementadas en una estructura con `Person:: function_that_implement_the_object()`.

```Rust

implt Person{
  fn new_person(first_name: String,
                last_name: String,
                age: i32,
                country: String) -> Person {
    return Person{first_name, last_name, age, country};
  }
}

fn main(){
  let person: Person = Person::new_person(first_name: "Peter".to_string(),
                                          last_name: "Parker".to_string(),
                                          age: 16,
                                          country: "EEUU".to_string())
  println!("{}", persona.first_name);
}
```

## Almacenamiento clave/valor o **HashMap**

Otra manera de almacenar información, además de las estructuras y los vectores, son los denominados **HashMap**.

Los mismos son **“diccionarios”** de datos, del tipo **clave/valor**, donde para acceder a un elemento, en lugar de utilizar el índice del mismo como en un vector, se utiliza la **Clave**, que puede ser un string o un número, para colocarle un nombre al **Valor** y obtenerlo.

Podemos guardar un nuevo valor con `insert()` y acceder al mismo a través de su **clave**.


```Rust
use std::collections::{HashMap};

//fn ma...
  let mut dictonary: HashMap<&str, &str> = HashMap::new();
  dictionary.inser("SuperHero1", "Spider Man");
  dictionary.inser("SuperHero2", "Hulk");
  dictionary.inser("SuperHero3", "Iron Man");

//...

```
## Accediendo al índice de un vector

Al recorrer un array con un `for`, resulta util a veces saber en que indice de la iteración nos encontramos. Esto lo hacemos con `.iter().enumerate()` y con ello podremos obtener el **index** como primer valor de un ciclo for, y el valor en cuestión en el segundo parámetro.

```Rust

//fn mai...

  let muy my_arr: Vec<String> = Vec::new();

  my_arr.push("1er valor".to_string());
  //my_arr.push("N valor".to_string());
  //my_arr.push(...

  for (index, word) in my_arr.iter().enumerate(){
    println!("{} {}", index, word);
  }
//...
```

## Option, Some y None

Estas tres palabras reservadas se utilizan mucho en Rust, si comprendemos bien su funcionamiento podemos sacarle el maximo jugo a Rust.

- `Option<T>` puede devolver:
  - `Some`, el cual es un valor determinado.
  - `None`, este es distinto a null. En Rust el valor nulo no existe.

Ejemplo: la función `dividir_numeros()` evaluará el denominador y, si este es cero, retornara un `None`, de lo contrario, devolverá el resultado de la división creando una variable `Some` que se devuelve luego de la operación.

```Rust

fn dividir_numeros(numerador: i128, denominador: i128) -> Option<i128> {
    if denominador == 0 {
        None
    } else {
        Some(numerador / denominador)
    }
}

```

Una forma de validar un `Option`, ya sea `Some` o `None` es usar la palabra reservada `Match`:

```Rust

fn main() {
    let result = dividir_numeros(10, 2);

    match result {
        Some(value) => { println!("El resultado es: {}", value) },
        None => { println!("No puedes dividir por cero"); },
    }
}

```

> Nota: Algunos caracteres caracteristicos de Rust vienen de C++ como:

> - `&`: pasajes por valor
> - `*`: referencia

