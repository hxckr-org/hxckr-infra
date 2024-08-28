{
  description = "Hxckr-infra development environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs { inherit system; };
      # Docker Compose configuration
      dockerComposeFile = pkgs.writeText "docker-compose.yml" ''
        services:
          soft_serve_postgres:
            container_name: soft-serve-db
            image: postgres:15-alpine
            restart: always
            environment:
              - POSTGRES_USER=$SOFT_SERVE_POSTGRES_USER
              - POSTGRES_PASSWORD=$SOFT_SERVE_POSTGRES_PASSWORD
              - POSTGRES_DB=$SOFT_SERVE_POSTGRES_DB
            ports:
              - 5432:5432
            volumes:
              - ''${SOFT_SERVE_POSTGRES_DATA}:/var/lib/postgresql/data
        volumes:
          soft_serve_postgres:
      '';
      # Function to start and stop the PostgreSQL containers
      startPostgres = ''
        docker-compose --file ${dockerComposeFile} up -d
        echo "PostgreSQL development database for Soft-Serve started"
      '';
      stopPostgres = ''
        docker-compose --file ${dockerComposeFile} down
      '';
    in {
      devShells.default = pkgs.mkShell {
        name = "hxckr-infra";
        buildInputs = with pkgs; [
          docker
          docker-compose
          rustup
          soft-serve
          diesel-cli
          openssl
          pkg-config
        ];
        shellHook = ''
          # Change the prompt color to blue when in the Nix shell
          export PS1="\[\033[01;34m\]\u@\h:\w\[\033[00m\]\$ "
          echo "You are now in the Nix shell!"
          # Load environment variables from .env file
          if [ -f ./.env ]; then
            set -a
            source ./.env
            set +a
          else
            echo "Warning: .env file not found in the current directory"
          fi
          mkdir -p "$SOFT_SERVE_POSTGRES_DATA"
          ${startPostgres}
          # Rust setup
          rustup default stable
          export PATH="$HOME/.cargo/bin:$PATH"
          echo "Rust is now available in the Nix shell!"
          # Soft-Serve database URL
          export SOFT_SERVE_DATABASE_URL="postgres://$SOFT_SERVE_POSTGRES_USER:$SOFT_SERVE_POSTGRES_PASSWORD@$SOFT_SERVE_POSTGRES_HOST:$SOFT_SERVE_POSTGRES_PORT/$SOFT_SERVE_POSTGRES_DB"
          echo "You can use this as your soft-serve database URL: $SOFT_SERVE_DATABASE_URL"
          # Clean up when shell is exited
          trap '${stopPostgres}' EXIT
        '';
      };
    });
}
