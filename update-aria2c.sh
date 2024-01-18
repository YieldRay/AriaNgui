#!/usr/bin/env bash

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)
cd "$script_dir" || exit

curl_fix() {
    curl -kL "$@"
}

download_link="https://github.com/aria2/aria2/releases/download/release-1.37.0/aria2-1.37.0-win-64bit-build1.zip"
curl_fix -O "$download_link"
file_name=$(basename "$download_link")
rm -rf ./src-tauri/binaries/*.exe
unzip -oq -j "$file_name" "${file_name%.*}/aria2c.exe" -d ./src-tauri/binaries/
rm "$file_name"
cd ./src-tauri/binaries/ || exit
mv aria2c.exe aria2c-x86_64-pc-windows-msvc.exe

echo "Successfully fetch $download_link"
