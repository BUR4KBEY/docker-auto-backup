#!/bin/bash

printenv > /etc/environment

: ${CRON_SYNTAX:='0 4 * * *'}

RUN_SCRIPT="cd /app && ./docker-auto-backup > /app/cron.log 2>&1"

# Import GPG public keys from /gpg folder
for pubkey_file in /gpg/*.gpg; do
  gpg --import "$pubkey_file"
done

# Ensure that the cron.log file exists
touch /app/cron.log

# Schedule the job
echo "$CRON_SYNTAX $RUN_SCRIPT" > /etc/crontab
crontab /etc/crontab

# Start cron daemon
service cron start

if [ "$RUN_AT_STARTUP" = "true" ]; then
  eval "$RUN_SCRIPT"
fi

# Keep the container running
tail -f /app/cron.log