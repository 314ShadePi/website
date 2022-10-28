#!/usr/bin/env sh
docker compose --profile deploy down --rmi all
docker compose --profile deploy up --force-recreate --build -d
