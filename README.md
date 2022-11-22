# Bot de Discord - Club ExDev
El bot de discord creado para el club ExDev (?

## Requisitos
Este bot fue construido con Rust, y el proyecto es manejado por Cargo. Para poder compilar este proyecto se necesitan ambas herramientas. Puedes descargarlas desde [el siguiente link](https://www.rust-lang.org/es).

Adicionalmente, es muy benéfico que estés familiarizada/o con la [Documentación de Discord](https://discord.com/developers/docs/intro). Échale una leída y pregunta a los que estén trabajando en este proyecto si tienes dudas, y quieres participar.

## Dependencias
Se utiliza el framework para bot de Discord ['Serenity'](https://github.com/serenity-rs/serenity), el cual tiene una fuerte dependencia de 'Tokio' para la construccion de funciones asincronas.

## Como compilar
De no haberse roto las dependencias, deberia ser posible compilar con el comando tipico de cargo:

```bash
$ cargo build
```

Para poder hacer correr el bot es necesario proveerle tu token de Discord. La forma de hacerlo es a traves de una variable de ambiente.
Para esto, copia el archivo .env.example con el nombre '.env', y rellena con los valores que correspondan. Luego, ejecuta el comando:

```bash
$ cargo run 
```

Recuerda que nunca deberias commitear tu archivo .env! esta incluido en el
gitignore por buena razon, y es que ahi es donde iran tus contraseñas o
credenciales de cualquier tipo.

## Como contribuir
Preguntale a cualquiera de los encargados de este proyecto qué es lo que está faltando. Si eres parte del club, deberías encontrar un thread en discord donde tenemos discusiones al respecto, y eventualmente un tablero Kanban con tareas pendientes.

Si no eres del club, y por alguna razón o casualidad deseas contribuir, por que quizás estás usando nuestro proyecto como base o inspiración para el tuyo, puedes preguntar a los encargados del club, quizás podamos incorporar a este proyecto.

En cualquiera de estos casos, el método de contruibir es a través de un Pull Request hacia la branch 'main'. Sin miedo ni asco! en caso de que haya un problema, sugerencias o felicitaciones en tu PR te lo comentaremos a traves de la misma. La idea es trabajar para que todo aporte forme parte de este proyecto.
