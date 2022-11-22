# discord_bot
El bot de discord creado para el club ExDev

## Requisitos
Este bot fue construido con Rust, y el proyecto es manejado por Cargo.
Para poder compilar este proyecto se necesitan ambas herramientas.
Puedes descargarlas desde [el siguiente link](https://www.rust-lang.org/es).

## Dependencias
Se utiliza el framework para bot de Discord 'Serenity', el cual tiene una 
fuerte dependencia de 'Tokio' para la construccion de funciones asincronas.

## Como compilar
De no haberse roto las dependencias, deberia ser posible compilar con
el comando tipico de cargo:

```bash
$ cargo build
```

Para poder hacer correr el bot es necesario proveerle tu token de Discord.
La forma de hacerlo es a traves de una variable de ambiente.
Para esto, copia el archivo .env.example con el nombre '.env', y rellena
con los valores que correspondan. Luego, ejecuta el comando:

```bash
$ cargo run 
```

Recuerda que nunca deberias commitear tu archivo .env! esta incluido en el
gitignore por buena razon, y es que ahi es donde iran tus contraseñas o
credenciales de cualquier tipo.
