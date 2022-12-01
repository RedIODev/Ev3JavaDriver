#!/bin/bash
rm -rf ./java/resources
mkdir -p ./java/resources
rm -rf ./java/build
mkdir -p ./java/build
cd java/src
zip -r Ev3Lib-sources.jar dev
mv Ev3Lib-sources.jar ../build/Ev3Lib-sources.jar
cd ../..
cd rust
echo Building rust
cross build --release
cp ./target/armv5te-unknown-linux-gnueabi/release/libev3.so ../java/resources/libev3.so
echo rust build finished. Building java...
cd ..
cmd.exe /c build.bat