#!/bin/bash

CHANNEL_ID="46839531-8153-4844-8bc2-6c003ecae6c3"
URL="http://127.0.0.1:8082/channels/${CHANNEL_ID}"

curl -v -i -X DELETE -H 'Content-Type: application/json' "${URL}"
