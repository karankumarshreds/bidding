#!/usr/bin/env bash

set -x # be explicit
set -e # exit if any step fails
set -o pipefail # use the error code for the failed step

# check if the cli tools are installed
if ! [ -x "$(which psql)" ]; then
    echo >&2 "Error: psql not installed"
    exit 1
fi
if ! [ -x "$(which sqlx)" ]; then
    echo >&2 "Error: sqlx not installed"
    echo >&2 "Use:"
    echo >&2 "cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
    echo >&2 "to install"
    exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRESS_PASSWORD:=password}
DB_NAME=${POSTGRES_DB:=bidding}
DB_PORT=${POSTGRES_PORT=5432}

echo "db user: ${DB_USER}"
echo "post user: ${POSTGRES_USER}"

sudo systemctl start postgresql
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -c "\q";
do
    >$2 echo "Postgres still unavailable, trying again..."
    sleep 1
done


echo "Postgres up and running on ${DB_PORT}"
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
echo "exported the database url"
echo $DATABASE_URL
sqlx database create




