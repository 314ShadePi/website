# 314ShadePi's website

## Deploying

Use `./deploy.sh` to deploy.
Remember to delete both the container **AND** the image when deploying multiple times.

## Development

1. Install Docker
2. Start the development environment with `docker compose --profile dev up`
3. Open VSCode
4. Develop

## Command List

- dev: `docker compose --profile dev up`
- build `docker compose --profile build up`
- deploy `./deploy.sh`
- setup: `./setup.sh`
