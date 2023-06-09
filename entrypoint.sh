#!/bin/bash

# Setup the Postgres schema
./substreams-sink-postgres setup ${POSTGRES_DSN} ./schema.sql

export ${SUBSTREAMS_API_TOKEN}

# Run the substreams-sink-postgres
./substreams-sink-postgres run ${POSTGRES_DSN} "${FIREHOSE_ENDPOINT}" ./token_substreams.spkg db_out --final-blocks-only