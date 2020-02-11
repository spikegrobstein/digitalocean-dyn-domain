# DigitalOcean Dynamic Domain

This tool will hit a special endpoint that returns the requests IP and compare to the statefile
`current_ip.txt`. If they differ, it will update the domain record on digital ocean via the API.
Otherwise, it will do nothing.

This requires that there is already a record in digital ocean and that everything is set up correctly on that
side.

## Environment Variables

 * `DO_API_TOKEN` -- the DigitalOcean API token
 * `DYN_HOSTNAME` -- the hostname that we will be updating the IP for
 * `IP_ENDPOINT` -- the URL for the endpoint that returns the IP (`https://example.com/my-ip`)
 * `STATEFILE` (optional) -- path to the statefile that the tool will use to know whether it needs to update
     the dns record. Default: `./current_ip.txt`

## Usage

```bash
    DO_API_TOKEN='xxxxxxxxxx' \
    DYN_HOSTNAME='home.example.com' \
    IP_ENDPOINT='https://example.com/my-ip' \
    ./digitalocean-dyn-domain
```

## nginx config

If you'd like to add your own endpoint to an nginx config, you can do so like this:

```nginx
# requests to /my-ip return the remote address
location /my-ip {
  add_header content-type "text/plain";
  return 200 $remote_addr;
}
```

> we add the `content-type` header so that browsers don't try to download the file

## Docker container

This repo also contains a `Dockerfile` in the `docker` directory. This will build a minimal Docker image that
contains just the binary and can execute the binary by default.

To build it, run the following from the root of the project:

    docker build -t "$TAG" -f docker/Dockerfile .

This will compile the binary using the currently pushed master branch from this repo.

Then execute with:

```bash
docker run \
  --env 'DO_API_TOKEN=XXXX' \
  --env 'DYN_HOSTNAME=home.example.com' \
  --env 'IP_ENDPOINT=https://example.com/my-ip' \
  --env 'STATEFILE=/state/current_ip.txt' \
  --volume './state:/state' \
  "$TAG"
```

## Disclaimer

This is not a great example of good Rust code. I slapped this together to do the bare minimum to use this for
myself and I may or may not iterate on this further. I'm still learning.

