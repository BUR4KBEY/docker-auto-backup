#!/bin/bash

: ${BACKUP_FOLDER_PATH:=./backup}

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

zstd -19 temp/backup.tar

gpg --always-trust --encrypt --recipient $GPG_RECIPIENT --out temp/backup.tar.zst.gpg temp/backup.tar.zst

mv temp/backup.tar.zst.gpg ./$DATE.tar.zst.gpg

rm -rf temp