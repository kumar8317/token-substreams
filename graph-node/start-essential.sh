#!/usr/bin/env bash

set -o allexport; 
source .env; 
set +o allexport; 
envsubst < config/config.tmpl > config/config.toml
docker-compose -f compose-indexer.yml up -d --build $@