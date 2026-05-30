# dondns-rs

![DonDNS-Rust](./res/dondns-rust.png)

A minimal CLI binary to update a [DonDominio](https://www.dondominio.com) dynamic DNS record via their JSON API.

> **Note:** If the hostname has multiple A records, only the first one will be updated.

## Build

```bash
cargo build --release
```

The binary will be at `target/release/dondns-rs`.

## Usage

Configure the following environment variables before running:

| Variable | Description |
|---|---|
| Variable | Required | Description |
|---|---|---|
| `DONDNS_USERNAME` | Yes | DonDominio account username |
| `DONDNS_PASSWORD` | Yes | DonDNS Key (found in DonDominio account preferences, not your account password) |
| `DONDNS_HOSTNAME` | Yes | Fully qualified domain name to update |
| `DONDNS_ADDRESS` | No | IP address to set; if omitted, the server resolves the caller's public IP |

Required variables must not be empty, may not exceed 256 characters, and may only contain alphanumeric characters, hyphens (`-`), or dots (`.`).

```bash
export DONDNS_USERNAME=myuser
export DONDNS_PASSWORD=mypassword
export DONDNS_HOSTNAME=home.example.com

./dondns
```

### Output

On success, a single line is printed to stdout and the process exits with code `0`:

```
OK: home.example.com updated
```

On failure, a single line is printed to stderr and the process exits with code `1`:

```
ERROR: Incorrect data
```
