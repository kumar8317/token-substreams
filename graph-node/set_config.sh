#!/usr/bin/env bash

set -o allexport; 
source .env; 
set +o allexport; 
envsubst < config/config.tmpl > config/config.toml