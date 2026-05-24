# KeyBox
Secure and simple storage for your API keys

# Run

```bash
$ cd KeyBox
$ export SECRET_KEY='...'
$ export GRAFANA_ADMIN_PASSWORD='...'
$ docker compose up --build
```

Grafana is exposed on `http://localhost:3001` with user `admin`. The password
comes from `GRAFANA_ADMIN_PASSWORD`. In GitHub Actions deployments set the
`GRAFANA_ADMIN_PASSWORD` repository secret.

# CLI

`KeyBox/keyboxctl` is a bash CLI for the HTTP API. It requires `curl`, `jq`,
`openssl`, and `base64`.

Generate a JWT locally from the same base64 `SECRET_KEY` used by the server:

```bash
export KEYBOX_URL=http://localhost:8001
export KEYBOX_SECRET_KEY='...'
export KEYBOX_TOKEN="$(./KeyBox/keyboxctl token --group admin --admin true)"
```

Use the API:

```bash
./KeyBox/keyboxctl ping
./KeyBox/keyboxctl create github ghp_xxx admin,dev
./KeyBox/keyboxctl list
./KeyBox/keyboxctl get github
./KeyBox/keyboxctl update github --value ghp_yyy --groups admin
./KeyBox/keyboxctl delete github
```
