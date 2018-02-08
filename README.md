# Fanta Cli

A cli to setup, and generate new components for, a Fanta app.

## Installation

- Clone this repository
- Run:
```
> cargo install --force
```

## Useage

### Creating a new project

This will create a new project using:
- [Fanta](https://github.com/trezm/fanta)
- Postgres
- [Diesel.rs](http://diesel.rs/)

```
> fanta-cli init ProjectName
```

*Note:* This installs the project pointing towards the default database of

```
DATABASE_URL=postgres://postgres@localhost/TestApp8
```

This can be updated at any time in the generated `.env` file.

### Creating a new component

```
> fanta-cli component User
> fanta-cli migrate
```

Running `migrate` is important because it generates schema populated from the database.

### Running the app

```
> cargo run
```

Just like a normal rust project.
