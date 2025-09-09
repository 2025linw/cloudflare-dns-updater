# Cloudflare DNS Updater

This programs is designed to be used with Cloudflare to update the IP address for an DNS entry pointed at a device that has a dynamically assigned IP address by an ISP.

There is no guarantee for the security of this program and it is made only to achieve the goal of updating the IP for an entry.

## Setup instructions

* This program assumes that the system has `rust` installed on the system, and has `crontab`.

1. `git clone` repository
2. Run `cp .env.example .env` and fill out variables.
3. Build release version of updater with `cargo b -r`
4. Create a `crontab` entry copying the format shown in the [crontab](#Crontab-Setup) section

### .env Setup

```sh
CLOUDFLARE_API_KEY="<API KEY>"
CLOUDFLARE_ZONE_ID="<ZONE ID>"
CLOUDFLARE_ACC_EMAIL="<EMAIL>"
DOMAIN_NAME="<DOMAIN NAME>"
```

### Crontab Setup

1. Open `crontab` editing using `crontab -e`
2. Add the following entries
   1. Setup automated updating:
      * `0 * * * * cd /path/to/repo/cloudflare-dns-updater && ./target/release/cloudflare_dns_updater >> /path/to/log/folder/cloudflare-dns-updater.log`
   2. Setup auto-removal of logs every month:
      * `0 * 1 * * cd /path/to/log/folder && > cloudflare-dns-updater.log`
