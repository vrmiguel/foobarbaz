# foobarbaz

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