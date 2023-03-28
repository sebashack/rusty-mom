#!/bin/bash

QUEUE_LABEL="QA"
TOPIC="T1"
URL="http://127.0.0.1:8082/queues/${QUEUE_LABEL}/channels/${TOPIC}"

curl -v -i -X PUT -H 'Content-Type: application/json' "${URL}"
