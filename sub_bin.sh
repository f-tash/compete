#!/bin/bash



cd "$(dirname "$0")"

# 引数の文字列を取得
input="$1"
# ハイフンで区切って配列に格納
IFS='-' read -ra parts <<< "$input"
# 変換されたパスを作成
original="./${parts[0]}/src/bin/${parts[1]}.rs"

destination_file="binary/main/src/main.rs"
# ファイルのコピー

cp $original $destination_file

cd binary/main
RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo +nightly build --target x86_64-unknown-linux-musl --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort  > output.txt 2>&1


base64 -i target/x86_64-unknown-linux-musl/release/main | tr -d '\n' > encoded_binary.txt

cat before.txt
cat encoded_binary.txt
cat after.txt

