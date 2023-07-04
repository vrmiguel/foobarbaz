<h2 align="center"><img align="center" src="https://github.com/vrmiguel/vrmiguel/assets/36349314/5170cc9d-e6bf-4e47-a7c0-e02c8778b8ec" height="70px" />  guana </h2>

## Project structure

`guana-grpc-server`
`guana-grpc-types`

## Installation

### Using `nix`

Start the Nix shell with:

```shell
# `cd` into the cloned repository
nix-shell --pure
```

You should now be able to interact with Postgres with `psql -p 5555 -U postgres`. The database will be shutdown and its data removed once the nix-shell exits.

## Migrations

Migrations can be run with the `migration` crate.

```shell
cd migration/
DATABASE_URL=postgresql://postgres@localhost:5555/postgres cargo run
```
