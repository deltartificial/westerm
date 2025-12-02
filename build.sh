#!/bin/bash
cd /Users/mat/Documents/Rust/westerm
MACOSX_DEPLOYMENT_TARGET="10.12" cargo build --release
make app
cp -r target/release/osx/Westerm.app /Applications/
echo "Build complete and installed to /Applications/"
