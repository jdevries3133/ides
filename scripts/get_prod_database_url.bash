#!/bin/bash

echo "$(
    kubectl get secret ides-secrets \
        -o jsonpath='{.data.DATABASE_URL}' \
    | base64 --decode \
    | sed 's/db-postgresql.ides.svc.cluster.local:5432/localhost:5433/g'
)"

