#!/bin/bash

export ROOT="$( readlink -f "$( dirname "${BASH_SOURCE[0]}" )" )"

MIGRATION_NAME=$1

cd ${ROOT}/..
cargo sqlx migrate --source ${ROOT}/sql add $MIGRATION_NAME
