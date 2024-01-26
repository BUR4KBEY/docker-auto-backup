#!/bin/bash

: ${BACKUP_FOLDER_PATH:=./backup}

if ! [[ "$ZSTD_COMPRESSION_LEVEL" =~ ^[0-9]+$ ]] || ((ZSTD_COMPRESSION_LEVEL < 1 || ZSTD_COMPRESSION_LEVEL > 22)); then
    ZSTD_COMPRESSION_LEVEL=19
fi

check_var() {
  local var_name="$1"

  if [ -z "${!var_name}" ]; then
    echo "\$$var_name is either unset or empty."
    exit 1
  fi
}

check_var "GPG_RECIPIENT"
check_var "DATE"

mkdir temp
cp -r $BACKUP_FOLDER_PATH temp

tar -cf temp/backup.tar -C temp backup

zstd "-$ZSTD_COMPRESSION_LEVEL" temp/backup.tar

gpg --always-trust --encrypt --recipient $GPG_RECIPIENT --out temp/backup.tar.zst.gpg temp/backup.tar.zst

mv temp/backup.tar.zst.gpg ./$DATE.tar.zst.gpg

rm -rf temp