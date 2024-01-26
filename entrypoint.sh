#!/bin/bash

# Start cron daemon
service cron start

# Schedule the job
echo "0 4 * * * /app/rust-backup-generator > /app/cron.log 2>&1" > /etc/crontab

if [ "$RUN_AT_STARTUP" = "true" ]; then
  /app/rust-backup-generator >> /app/cron.log 2>&1
fi

# Keep the container running
tail -f /app/cron.log