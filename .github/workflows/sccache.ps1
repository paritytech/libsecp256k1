#!/usr/bin/env pwsh
$OS=$args[0]
$SCCACHE_CACHE_SIZE="1G"
$SCCACHE_IDLE_TIMEOUT=0
$SCCACHE_DIR="$HOME/sccache"
$VERSION="0.2.13"
echo "Current OS:" $OS
switch ($OS){
   "macOS" {$PLATFORM = "x86_64-apple-darwin"}
   "Linux" {$PLATFORM = "x86_64-unknown-linux-musl"}
   "Windows"  {$PLATFORM ="x86_64-pc-windows-msvc"}
}
echo "Target arch: " $PLATFORM
$BASENAME = "sccache-$VERSION-$PLATFORM"
$URL = "https://github.com/mozilla/sccache/releases/download/"+"$VERSION/$BASENAME.tar.gz"
echo "Download sccache from "+$URL
curl -LO $URL
tar -xzvf "$BASENAME.tar.gz"
ls $BASENAME/
. $BASENAME/sccache --start-server
echo "$(pwd)/$BASENAME" >> "$GITHUB_PATH"
echo $GITHUB_PATH
echo "RUSTC_WRAPPER=sccache" >> "$GITHUB_ENV"
echo $GITHUB_ENV
