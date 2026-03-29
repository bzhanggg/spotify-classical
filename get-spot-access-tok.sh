#!/bin/bash

source .env

TOKEN=$(curl -X POST "https://accounts.spotify.com/api/token" \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d "grant_type=client_credentials&client_id=${CLIENT_ID}&client_secret=${CLIENT_SECRET}" \
    | jq -r '.access_token')

export SPOTIFY_ACCESS_TOKEN="$TOKEN"
echo "Token set as SPOTIFY_ACCESS_TOKEN=${SPOTIFY_ACCESS_TOKEN}"