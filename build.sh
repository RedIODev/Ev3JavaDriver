#!/bin/bash
rm -rf ./java/resources
mkdir -p ./java/resources
cd rust
echo Building rust
cross build --release
cp ./target/armv5te-unknown-linux-gnueabi/release/libev3.so ../java/resources/libev3.so
echo rust build finished. Building java...
cd ..
cmd.exe /c build.bat