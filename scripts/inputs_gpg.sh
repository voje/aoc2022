#!/bin/bash

enc_pass_file=".inputs_enc_pass.txt"

function help() {
  echo "Put your password in $enc_pass_file, then run either:"
  echo "$ $0 encrypt"
  echo "$ $0 decrypt"
}

if [ ! -f "$enc_pass_file" ]; then
  echo "[*] Put your encryption password (aoc2022_inputs_enc) in $enc_pass_file"
  help
  exit 1
fi

function encrypt_one() {
  echo "Encrypting: $1"
  gpg --symmetric --batch --passphrase-file $enc_pass_file "$1"
}

function decrypt_one() {
  dec_f="${1//.gpg/}"
  echo "Decrypting: $1 as $dec_f"
  gpg -d --batch --passphrase-file $enc_pass_file $1 > "$dec_f"
}

case $1 in
  "encrypt")
    # find ./inputs/ -name "*.txt" -exec encrypt_one \;
    find ./inputs/ -type f -name "*.txt" | while read -r file; do encrypt_one "$file"; done
    ;;
  "decrypt")
    # find ./inputs/ -name "*.txt.gpg" -exec decrypt_one \;
    find ./inputs/ -type f -name "*.txt.gpg" | while read -r file; do decrypt_one "$file"; done
    ;;
  *)
    help
    exit 1
    ;;
esac


