#!/bin/bash

QUEUE_LABEL="QA"
URL="http://127.0.0.1:8082/queues/${QUEUE_LABEL}"

curl -v -i -X POST -H 'Content-Type: application/json' "${URL}"
