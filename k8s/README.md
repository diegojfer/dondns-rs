![DonDNS-Rust](../res/Kubernetes_logo.png)

# Kubernetes Deployment

This directory contains manifests to run `dondns-rs` as a Kubernetes CronJob. It is designed for small personal lab clusters where nodes have dynamic public IP addresses.

## How it works

The CronJob runs every 5 minutes. Each execution calls the DonDominio API without supplying an explicit IP address, so the API automatically resolves the public IP of the caller — which is the node running the pod. That IP is then written to the DNS A record for your hostname, keeping it in sync as the node's IP changes over time.

## Prerequisites

- A Kubernetes cluster (single-node or multi-node) whose nodes have a routable public IP.
- A [DonDominio](https://www.dondominio.com) account with a hostname configured for dynamic DNS.
- Your **DonDNS Key** (found in DonDominio account preferences under the API / DynDNS section — this is not your account password).
- `kubectl` configured to talk to the cluster.

## Configuration

Edit `dondns-rs.yaml` and fill in the `Secret` fields before applying:

| Field | Description |
|---|---|
| `DONDNS_USERNAME` | Your DonDominio account username |
| `DONDNS_PASSWORD` | Your DonDNS Key |
| `DONDNS_HOSTNAME` | The fully qualified hostname to update (e.g. `home.example.com`) |

```yaml
stringData:
  DONDNS_USERNAME: "your-username"
  DONDNS_PASSWORD: "your-dondns-key"
  DONDNS_HOSTNAME: "home.example.com"
```

> The secret values are stored in plain text inside the YAML while editing. Once applied, Kubernetes base64-encodes them. Do not commit the file with real credentials.

## Deploy

```bash
kubectl apply -f dondns-rs.yaml
```

This creates:
- A `Secret` named `dondns-rs-secret` holding the credentials.
- A `CronJob` named `dondns-rs` that runs every 5 minutes.

## Verify

Check that the CronJob was created:

```bash
kubectl get cronjob dondns-rs
```

Trigger a manual run to confirm credentials and connectivity are correct:

```bash
kubectl create job --from=cronjob/dondns-rs dondns-rs-test
```

Inspect the output:

```bash
kubectl logs -l job-name=dondns-rs-test
```

A successful run prints:

```
OK: home.example.com updated
```

## Adjusting the schedule

The default schedule is every 5 minutes (`*/5 * * * *`). Edit the `schedule` field in `dondns-rs.yaml` to change it. For clusters where the IP changes infrequently, a longer interval (e.g. every 15 or 30 minutes) reduces noise in the job history.

## Cleanup

```bash
kubectl delete -f dondns-rs.yaml
```
