#!/bin/bash

rm -rf dist/*

npm run build

docker build -t notmattlucas/habot-vue:$1 .

docker push notmattlucas/habot-vue:$1