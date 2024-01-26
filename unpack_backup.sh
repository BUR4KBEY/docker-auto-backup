#!/bin/bash

if [[ $1 != *.tar.zst.gpg ]]; then
    echo "Invalid file format. Expected .tar.zst.gpg extension."
    exit 1;
fi

gpg_out=$(basename "$1" .gpg)

gpg --decrypt --output $gpg_out $1

zstd -d $gpg_out

zst_out=$(basename "$gpg_out" .zst)

tar -xf $zst_out

rm $gpg_out
rm $zst_out