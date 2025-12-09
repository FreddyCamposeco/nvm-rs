#!/bin/bash

API_URL="https://api.github.com/repos/FreddyCamposeco/nvm-rs/releases/latest"

echo "=== Fetching GitHub API response ==="
RESPONSE=$(curl -fsSL "$API_URL" -H "User-Agent: nvm-rs-installer")

echo "Raw response:"
echo "$RESPONSE"
echo ""
echo "=== Formatted with jq (if available) ==="
if command -v jq &> /dev/null; then
    echo "$RESPONSE" | jq '.'
else
    echo "jq not available"
fi

echo ""
echo "=== Trying to extract tag_name with grep ==="
echo "Pattern 1 (with spaces): '\"tag_name\": \"[^\"]*\"'"
echo "$RESPONSE" | grep -o '"tag_name": "[^"]*"'

echo ""
echo "Pattern 2 (without spaces): '\"tag_name\":\"[^\"]*\"'"
echo "$RESPONSE" | grep -o '"tag_name":"[^"]*"'

echo ""
echo "=== Asset names in response ==="
echo "$RESPONSE" | grep -o '"name":"[^"]*"' | head -10
