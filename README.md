# thruster-cli

A cli to setup, and generate new components for, a thruster app. Still a work in progress, so please report bugs!

## Installation

- Clone this repository
- Run:
```
> cargo install --path . --force
```

## Quick Start

To make a simple project with a "Messages" model, simply run:

```
> thruster-cli init test-project
> thruster-cli component message
> thruster-cli migrate
```

To develop locally (i.e. faster build,) run

```
> docker-compose up -d postgres
> cargo run
```

Or, to use docker entirely, you can run

```
> docker-compose up
```

## Useage

### Creating a new project

This will create a new project using:
- [thruster](https://github.com/trezm/thruster)
- Postgres
- [Diesel.rs](http://diesel.rs/)
- Relevant docker files

```
> thruster-cli init ProjectName
```

*Note:* This installs the project pointing towards the default database of

```
DATABASE_URL=postgres://postgres@localhost/ProjectName
```

This can be updated at any time in the generated `.env` file.

### Creating a new component

```
> thruster-cli component User
> thruster-cli migrate
```

Running `migrate` is important because it generates schema populated from the database.

### Running the app

The whole app is initialized using docker and docker-compose, so you can simply run:
```
> docker-compose up
```

If you prefer to run outside of a container, you can run
```
> cargo run
```

Just like a normal rust project. Make sure you have postgres running as well so your server has a DB!

## Using for async

Commands are the same, just add `--async` as a flag! For example,

```
> thruster-cli init --async ProjectName
> thruster-cli component --async SomeComponent
```
