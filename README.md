# besui
Multichain triggers and actions workflow

## Getting started
To get started, install [cargo-make](https://github.com/sagiegurari/cargo-make) on your local machine to run tasks which defined in `Makefile.toml`. Then create your own `.env` file in your local workspace with valid values (you can use default values in `env.example`).

## Writing database schemas and generate entities process
We use [Sea-orm](https://www.sea-ql.org/SeaORM/) to write database schemas and generate entities.
We write migration to create tables, indexes and then we can generate entites from the database.