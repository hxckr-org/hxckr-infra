# Git Service Project

This folder host the codebase that provides a web service to interact with soft serve git server, allowing for soft serve command to be accesible over http.

## Prerequisites

- Nix package manager
- Rust (will be installed via Nix)
- Soft Serve git server
- Postgres (provided via nix setup using docker)
- Docker and Docker Compose (for Postgres)

## Setup

### 1. Configure the Project

Clone the project repository:

```bash
git clone [https://github.com/Extheoisah/hxckr-infra.git](https://github.com/Extheoisah/hxckr-infra.git)
cd hxckr-infra
```

Run `cp .env.example .env` to create a `.env` file.
> Adjust these values as needed to match your configuration.

### 2. Configure Nix
> If you haven't installed Nix yet, please visit: `https://nixos.org/download/`

> We use flakes to manage the project dependencies. To enable flakes after installing nix, add
```bash
experimental-features = nix-command flakes
```
> to your `~/.config/nix/nix.conf` file.


### 3. Set up Soft Serve
Install Soft Serve:
For development, a soft serve executable has been provided in the nix setup for the project so that you don't have to install it separately.
### 3.1. Before initializing Soft Serve
Before initializing Soft Serve, you need to set up the SSH key that will be used to authenticate with the server. This key should be added to the authorized keys in Soft Serve.
> Generate new ed_25519 key pair using the following command:

```bash
 ssh-keygen -t ed25519 -C "your email or any identifier"
```
This will generate a new SSH key pair in the default location (`~/.ssh/id_ed25519` and `~/.ssh/id_ed25519.pub`).
View the public key using the following command:

```bash
cat ~/.ssh/id_ed25519.pub
```
> Copy the public key and add it to `SOFT_SERVE_INITIAL_ADMIN_KEYS` in your .env file.
> Next is to set config to use your private key for the server. Add the following to your `~/.ssh/config` file:
```bash
Host soft
  HostName localhost
  Port 23231
  IdentityFile ~/.ssh/id_ed25519
  IdentitiesOnly yes
```
Initialize Soft Serve:
```bash
soft serve
```
> This will start the Soft Serve server on `localhost:23231`.
### 3.2. After initializing Soft Serve
> After initializing Soft Serve, the default DB is SQLite.
> You can change the DB to Postgres by setting the `SOFT_SERVE_POSTGRES_DB` in the `.env` file.
> Use this command to view the soft serve postgres provided in the nix setup: `echo @SOFT_SERVE_DATABASE_URL`, this will show the postgres url to use in the `.env` file.
> Copy this url and set it soft serve data directory created when you first run soft serve.
> You should have something like this in `data/config.yaml`
```bash
# The database configuration.
db:
  # The database driver to use.
  # Valid values are "sqlite" and "postgres".
  driver: "postgres"
  # The database data source name.
  # This is driver specific and can be a file path or connection string.
  # Make sure foreign key support is enabled when using SQLite.
  data_source: "postgres://soft_serve:soft_serve@0.0.0.0:5432/soft_serve?sslmode=disable"
```


## Setup git-service
### Build and Run the Project
In base directory of the project, run the following commands(also exports all the env variable needed for development):
```bash
nix develop
```

Start the soft serve server in nix shell:
```bash
soft serve
```

In another nix shell (you enter nix shell after running `nix develop` from the codebase base directory) cd into the `git-service` directory:
```bash
cd git-service
```
Run the soft serve wrappper to expose over http:
```bash
cargo run
```
The server should now be running on `http://127.0.0.1:8080`.

## Usage

The server provides the following endpoints:

1. Test Connection:
```bash
curl http://127.0.0.1:8080
```

2. Create User:
```bash
curl -X POST http://127.0.0.1:8080/create_user \
  -H "Content-Type: application/json" \
  -d '{"username": "testuser"}'
```

3. Create Token:
```bash
curl -X POST http://127.0.0.1:8080/create_token \
  -H "Content-Type: application/json" \
  -d '{"token_name": "testtoken"}'
```

4. Create Repository:
```bash
curl -X POST http://127.0.0.1:8080/create_repo \
  -H "Content-Type: application/json" \
  -d '{"repo_name": "testrepo", "repo_url": "https://github.com/user/repo.git"}'
```

## Development
To format your code run this in git-service directory:
```bash
cargo fmt
```

## Troubleshooting
If you encounter any issues with SSH connections, ensure that:
1. Soft Serve is running
2. The SSH key specified in `SOFTSERVE_KEY_PATH` exists and has the correct permissions
3. The SSH key is added to Soft Serve's authorized keys
