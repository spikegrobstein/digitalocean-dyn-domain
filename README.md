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

## Usage

```bash
    DO_API_TOKEN='xxxxxxxxxx' \
    DYN_HOSTNAME='home.example.com' \
    IP_ENDPOINT='https://example.com/my-ip' \
    ./digitalocean-dyn-domain
```

## Disclaimer

This is not a great example of good Rust code. I slapped this together to do the bare minimum to use this for
myself and I may or may not iterate on this further. I'm still learning.

