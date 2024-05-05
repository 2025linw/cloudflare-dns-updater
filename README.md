# Cloudflare DNS Updater

This programs is designed to be used with Cloudflare to update the IP address for an DNS emtry pointed at a device that has a dynamically assigned IP address.

There is no guarentee for the security of this program and it is made only to achieve the goal of updating the IP for an entry.

## Setup instructions

* This program assumes that the system has `rust` installed on the system, and has `crontab`.

1. `git clone` repository
2. Make a copy of `.env` and enter details.
3. Run `cargo b -r`
4. Create a `crontab` entry copying the format shown in the [crontab](#Crontab-Setup) section

### .env Setup


### Crontab Setup

1. Setup automated updating
*  `0 * * * * cd <path to repo> && ./target/release/cloudflare_dns_updater >> <path to log folder>/cloudflare-dns-updater.log`
2. Setup autoclearing of logs every month
* `0 * 1 * * cd <path to log folder> && > cloudflare-dns-updater.log`
