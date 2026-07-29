#!/bin/sh
# VigilantAI JWT Key Generation (Unix/Linux/macOS)
set -e

OUTPUT_DIR="${1:-./keys}"
mkdir -p "$OUTPUT_DIR"

echo "=== VigilantAI JWT Key Generation ==="
echo ""

PRIVATE_KEY="$OUTPUT_DIR/private_key.pem"
PUBLIC_KEY="$OUTPUT_DIR/public_key.pem"

echo "Generating RSA 2048-bit private key..."
openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 -out "$PRIVATE_KEY" 2>/dev/null
echo "Generating RSA 2048-bit public key..."
openssl pkey -in "$PRIVATE_KEY" -pubout -out "$PUBLIC_KEY" 2>/dev/null

echo "Private key: $PRIVATE_KEY"
echo "Public key:  $PUBLIC_KEY"
echo ""

# Generate single-line versions for .env
PRIVATE_KEY_LINE=$(awk 'NF {sub(/\r/, ""); printf "%s\\n",$0}' "$PRIVATE_KEY")
PUBLIC_KEY_LINE=$(awk 'NF {sub(/\r/, ""); printf "%s\\n",$0}' "$PUBLIC_KEY")

echo "=== Environment Variables for .env ==="
echo "JWT_PRIVATE_KEY=$PRIVATE_KEY_LINE"
echo "JWT_PUBLIC_KEY=$PUBLIC_KEY_LINE"
echo ""
echo "Key generation complete!"
echo "Copy the JWT_PRIVATE_KEY and JWT_PUBLIC_KEY values above into your .env file."
