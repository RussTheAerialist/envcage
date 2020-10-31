#!/usr/bin/env bash 

docker run -d \
  --rm \
  --name dev-postgres \
  -e POSTGRES_PASSWORD=testdb1234! \
  -v $(dirname $0)/postgres-test-data \
  -p 5433:5432 postgres
