#!/usr/bin/env bash

set -o allexport; 
source .env; 
set +o allexport; 
envsubst < config/config.tmpl > config/config.toml
docker-compose -f compose-db.yml  -f compose-indexer.yml -f compose-monitoring.yml up -d --build $@