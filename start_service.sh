#!/bin/sh
pg_sse &
envoy -c /etc/service-envoy.yaml --service-cluster ${SERVICE_NAME}