# Docker Auto Backup 📦

Docker Auto Backup is a lightweight, Rust-based automatic backup generator designed for seamless integration with cloud providers such as [Google Drive](https://www.google.com/drive) and [BackBlaze B2](https://www.backblaze.com/cloud-storage).

## Available Cloud Providers 🌐

As of now, Docker Auto Backup supports [BackBlaze B2](https://www.backblaze.com/cloud-storage), and I'm actively working on implementing [Google Drive](https://www.google.com/drive).

- [x] [BackBlaze B2](https://www.backblaze.com/cloud-storage)
- [ ] [Google Drive](https://www.google.com/drive)

## Motivation 🌱

I'm currently on a Rust-learning journey and have recently acquired a Raspberry Pi 5 for hosting various services using Docker within my home. The motivation behind this project is to create a cloud-backup system capable of generating encrypted backups and automatically uploading them.

Then, I chose to incorporate Rust into this project as a means of self-improvement. However, the tasks of generating and encrypting backups are currently handled by bash scripts. My decision to use bash was driven by the belief that working with binaries is more straightforward in bash than in Rust. Presently, only the upload process is implemented using Rust.

Given my newcomer status in Rust, I welcome pull requests for both incorporating best practices and implementing future improvements.

## How the System Works 🛠️

We use Docker containers to run this project, following these steps:

1. A designated backup folder, typically `/backup`, serves as the repository. Whatever data we want to back up must be mounted to this folder, like so:

   ```yaml
   services:
     app:
       # ...
       volumes:
         - /path/to/data/we/want/to/backup:/backup/service-1:ro
   ```

2. Firstly, our Rust application spawns a `temp` folder, an identical copy of the `/backup` folder, for both compression and encryption purposes.

3. Then, we employ `tar` to combine all files and folders into a single binary.

4. Following that, we utilize `zstd` to compress the tar file.

5. Post-compression, we employ `gpg` to encrypt the compressed tar file.

6. The next stop involves uploading the encrypted compressed tar file to cloud providers like [Google Drive](https://www.google.com/drive) and [BackBlaze B2](https://www.backblaze.com/cloud-storage).

7. To wrap things up, a cleanup process is initiated, tidying up the aftermath of the backup generation.

This entire sequence of operations is automated through cronjobs.

## Deploying 🚀

1. Create a `docker-compose.yml` file:

   ```yml
   services:
     app:
       image: burakbey/docker-auto-backup:latest
       container_name: docker-auto-backup
       restart: unless-stopped
       environment:
         BACKUP_FOLDER_PATH: /backup # optional, you must mount to /app/backup if you don't set
         BACKBLAZE_KEY_ID: ${BACKBLAZE_KEY_ID}
         BACKBLAZE_APPLICATION_KEY: ${BACKBLAZE_APPLICATION_KEY}
         BACKBLAZE_BUCKET_REGION: ${BACKBLAZE_BUCKET_REGION}
         BACKBLAZE_BUCKET_NAME: ${BACKBLAZE_BUCKET_NAME}
         GPG_RECIPIENT: ${GPG_RECIPIENT}
         # NTFY_URL: https://ntfy.sh/example # optional
         # NTFY_CA_FILE_PATH: /certs/ca.pem # optional, used to setting up a custom ca file while requesting to the ntfy server
         # CRON_SYNTAX: '0 4 * * *' # optional, default value is provided
         # RUN_AT_STARTUP: false # optional, default value is provided
         # DO_NOT_CLEANUP: false # optional, default value is provided
         # ZSTD_COMPRESSION_LEVEL: 19 # optional, default value is provided, min 1, max 22
       volumes:
         - /etc/timezone:/etc/timezone:ro
         - /etc/localtime:/etc/localtime:ro
         - ../service-1/data:/backup/service-1:ro # /backup must be same as environment.BACKUP_FOLDER_PATH
         - /path/to/things_should_be_backed_up:/backup/things:ro
         - ./gpg:/gpg:ro
         # - ./ca.pem:/certs/ca.pem # optional, check `NTFY_CA_FILE_PATH` environment variable
   ```

   Additionally, if you prefer not to expose sensitive information directly in the compose file (recommended), create a `.env` file as follows:

   ```
   BACKBLAZE_KEY_ID=""
   BACKBLAZE_APPLICATION_KEY=""
   BACKBLAZE_BUCKET_REGION=""
   BACKBLAZE_BUCKET_NAME=""
   GPG_RECIPIENT=""
   ```

   Obtain your [BackBlaze B2](https://www.backblaze.com/cloud-storage) credentials by following the instructions [here](https://www.backblaze.com/apidocs/introduction-to-the-s3-compatible-api).

   Generate a GPG key on your local machine (referred to as the host machine later). For details, a quick search should guide you. Then, put your **GPG Key ID** into the `GPG_RECIPIENT` environment variable.

   Export your GPG key's public key, create a folder named `gpg` inside the compose file's directory, and place your `public_key.gpg` file into that folder. The name is arbitrary and does not matter.

   The final project tree should resemble:

   ```
   compose-project/
   ├── gpg/
   │   └── public_key.gpg
   ├── .env
   └── docker-compose.yml
   ```

2. Deploy the stack:

   ```bash
   docker compose up -d
   ```

   The application should now be operational. You can test it by either setting the `RUN_AT_STARTUP` environment variable to true and checking the logs with the command `docker-compose logs app -f`, or you can execute the following command to test:

   ```bash
   docker compose exec -it app /app/docker-auto-backup
   ```

## Direct Backups From Containers ⭐

Introducing a new feature, Docker Auto Backup now supports generating backups directly from within Docker containers, with added support for pre/post script execution.

The rationale behind this feature is to accommodate scenarios where specific commands need to be executed to generate the data required for backup. For instance, consider a PostgreSQL container where you aim to perform periodic backups. Mounting the `/var/run/postgres/data` directory directly might include unnecessary files, bloating the backup size. Instead, generating a dump using the `pg_dump` command and backing up the resulting SQL file is a more efficient approach.

To utilize this feature, begin by creating a `config.yml` file with the following contents:

```yml
containers:
  - name: service-1
    files:
      - /data/.:/backup/service-1/data

  - name: service-2
    files:
      - /data/.:/backup/service-2/data
    pre_build_script: |
      touch /root/logs
      echo "pre-build script ran" >> /root/logs
    post_build_script: |
      echo "post-build script ran" >> /root/logs
```

In this configuration file:

- For the container named `service-1`, the files located at `/data/.` are retrieved and mounted to `/backup/service-1/data` within the backup container.

- For the container named `service-2`, the specified pre-build script is executed first (`touch /root/logs` and `echo "pre-build script ran" >> /root/logs`), followed by retrieving the files at `/data/.` and mounting them to `/backup/service-2/data` within the backup container. Additionally, the post-build script (`echo "post-build script ran" >> /root/logs`) is executed in the `service-2` container as part of the cleanup process.

> [!NOTE]  
> The reason we use `/data/.` instead of `/data` is that the retrieval process depends on the `docker cp` command. Imagine you want to mount the `/data` folder to `/backups/service-1`. If you don't use `/data/.`, it will mount to `/backups/service-1/data` instead of `/backups/service-1`.

To use this configuration file, mount it to the container along with the Docker socket:

```yml
services:
  app:
    # other configurations
    volumes:
      # other volumes
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./config.yml:/app/config.yml:ro
```

## Development 🔧

Setting up your Rust development environment is pretty standard. However, to streamline the process and spare you from constantly running the `cargo run` command with a slew of environment variables, you can use `./dev.sh`.

To set it up:

1. Copy the `dev.example.sh` file and rename it to `dev.sh`.

2. Provide the necessary environment variables in `dev.sh` based on your preferences.

3. Make the script executable:
   ```bash
   chmod +x dev.sh
   ```

## Unpacking the Encrypted Backup 🔐

Imagine you've downloaded a backup, and now you're ready to unpack it. Sure, you could type a bunch of commands, but here's a simpler way:

Run the `./unpack_backup.sh backup_file.tar.zst.gpg` script on your host machine. You'll be prompted to enter your passphrase. Once done, you should find a `backup` folder in your current working directory.

## ☕ Support

If you find this project useful and would like to support [me](https://github.com/BUR4KBEY), you can do so by visiting [my website](https://burakbey.dev).

<a href="https://burakbey.dev" target="_blank"><img src="https://burakbey.dev/github_support_snippet.png" style="height: 56px !important;width: 200px !important;" alt="Buy me a coffee"></img></a>
