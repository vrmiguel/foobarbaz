
with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "foobarbaz";

    buildInputs = [
        glibcLocales
        postgresql
        pgcli
        openssl.dev
        pkgconfig
        pkgs.sea-orm-cli
    ];

    OPENSSL_DEV=openssl.dev;
    PGDATA = "${toString ./.}/.pg";

    shellHook = ''
        echo "Using ${postgresql_12.name}."

        # Setup: other env variables
        export PGHOST="$PGDATA"
        # Setup: DB
        [ ! -d $PGDATA ] \
          && pg_ctl initdb -o "-U postgres" \
          && cat "$postgresConf" >> $PGDATA/postgresql.conf
        pg_ctl -o "-p 5555 -k $PGDATA" start

        function end {
          echo "Shutting down the database..."
          pg_ctl -D $PGDATA stop
          echo "Removing directories..."
          rm -rf $PGDATA .data
        }
        trap end EXIT
    '';
}