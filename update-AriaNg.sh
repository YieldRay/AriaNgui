#!/usr/bin/env bash

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)
cd "$script_dir" || exit

curl_fix() {
    curl -kL "$@"
}

release_info=$(curl_fix -s https://api.github.com/repos/mayswind/AriaNg/releases/latest)
download_link=$(echo "$release_info" | grep browser_download_url | head -n 1 | cut -d '"' -f 4)
curl_fix -O "$download_link"
file_name=$(basename "$download_link")
rm -rf ./dist/*
unzip -oq "$file_name" -j index.html -d ./dist
rm "$file_name"

echo "Successfully fetch $download_link"
