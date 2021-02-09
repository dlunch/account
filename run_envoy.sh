#!/bin/sh
docker run --network host -v $(pwd)/docker/envoy.yaml:/etc/envoy/envoy.yaml envoyproxy/envoy:v1.17-latest
