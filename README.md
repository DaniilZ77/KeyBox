# KeyBox
Secure and simple storage for your API keys

# Run

```bash
$ cd KeyBox && docker compose up --build
```

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
