#!/bin/bash

URL="http://127.0.0.1:8082/channels/"

curl -v -i -X GET -H 'Content-Type: application/json' "${URL}"
