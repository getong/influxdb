#!/bin/bash -e

/usr/bin/influxd -config /etc/influxdb/influxdb.conf $INFLUXD_OPTS &
echo $! > /var/lib/influxdb/influxd.pid

BIND_ADDRESS=$(influxd config | grep -A5 "\[http\]" | grep '^  bind-address' | cut -d ' ' -f5 | tr -d '"')
HOST=${BIND_ADDRESS%%:*}
HOST=${HOST:-"localhost"}
PORT=${BIND_ADDRESS##*:}

set +e
result=$(curl -s -o /dev/null http://$HOST:$PORT/health -w %{http_code})
while [ "$result" != "200" ]; do
  sleep 1
  result=$(curl -s -o /dev/null http://$HOST:$PORT/health -w %{http_code})
done
set -e