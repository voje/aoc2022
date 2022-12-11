#!/bin/bash

enc_secret_file="inputs_enc_pass.txt"
if ![ -f encryption_secret.txt ]; then
  echo "[*] Put your encryption password (aoc2022_inputs_enc) in $enc_secret_file"
  exit 1
fi

for input in inputs/*.txt; do
  echo "[*] Encrypting: $input"
l
done

