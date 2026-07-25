#!/bin/sh
set -e

if [ -z "$JWT_PRIVATE_KEY" ]; then
  export JWT_PRIVATE_KEY="$(cat /app/keys/private_key.pem)"
fi

if [ -z "$JWT_PUBLIC_KEY" ]; then
  export JWT_PUBLIC_KEY="$(cat /app/keys/public_key.pem)"
fi

exec /app/backend
