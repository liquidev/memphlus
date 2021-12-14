#!/usr/bin/env bash

# compile-assets.sh
# Recompiles assets that need to be compiled, such as maps.

generated_assets=assets/generated
tileset_json="$generated_assets/tileset.json"
map_json="$generated_assets/map.json"

echo "Exporting tileset"
tiled --export-tileset json res/map/tileset.tsx "$tileset_json"
echo "Exporting map"
tiled --export-map json res/map/map.tmx "$map_json"

compress-json() {
   cat "$1" | jq -c | tee "$1" > /dev/null
}

echo "Compressing tileset"
compress-json "$tileset_json"
echo "Compressing map"
compress-json "$map_json"
