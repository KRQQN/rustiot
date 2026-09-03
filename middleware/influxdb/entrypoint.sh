#!/bin/sh
set -eu

DATA_DIR="/var/lib/influxdb3"
TOKEN_FILE="$DATA_DIR/influx-token.json"

echo "Using data directory: $DATA_DIR"

mkdir -p "$DATA_DIR"

# Generate admin token on first startup
if [ ! -f "$TOKEN_FILE" ]; then
  echo "Generating InfluxDB admin token..."

  influxdb3 create token \
    --admin \
    --name middleware \
    --offline \
    --output-file "$TOKEN_FILE"

  if [ ! -f "$TOKEN_FILE" ]; then
    echo "ERROR: Token was not created."
    exit 1
  fi

  echo "InfluxDB token generated."
else
  echo "InfluxDB token already exists."
fi

# Read token from file
TOKEN=$(sed -n 's/.*"token"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' "$TOKEN_FILE")

if [ -z "$TOKEN" ]; then
  echo "ERROR: Could not read token."
  exit 1
fi

echo "Starting InfluxDB..."

influxdb3 serve \
  --node-id=node0 \
  --object-store=file \
  --data-dir="$DATA_DIR/data" \
  --admin-token-file="$TOKEN_FILE" &

INFLUX_PID=$!

echo "Waiting for InfluxDB..."

# /health requires authentication, so include the token
until curl -sf \
  -H "Authorization: Bearer $TOKEN" \
  http://localhost:8181/health \
  >/dev/null; do
  if ! kill -0 "$INFLUX_PID" 2>/dev/null; then
    echo "ERROR: InfluxDB stopped."
    exit 1
  fi

  sleep 1
done

echo "InfluxDB is ready."

echo "Creating telemetry database..."

influxdb3 create database telemetry \
  --host http://localhost:8181 \
  --token "$TOKEN" \
  2>&1 || echo "telemetry may already exist."

echo "InfluxDB initialization complete."

wait "$INFLUX_PID"
