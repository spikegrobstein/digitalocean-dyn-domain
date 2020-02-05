## Set up the project and build it for release

FROM ekidd/rust-musl-builder:1.41.0
COPY --chown=rust . /home/rust/src/digitalocean-dyn-domain
WORKDIR /home/rust/src/digitalocean-dyn-domain
RUN cargo build --release

## Build the actual container

FROM scratch

COPY --from=0 /home/rust/src/digitalocean-dyn-domain/target/x86_64-unknown-linux-musl/release/digitalocean-dyn-domain /digitalocean-dyn-domain

# Add the cacerts file so that we can make https calls from our program
COPY docker/files/cacert.pem /cacert.pem
ENV SSL_CERT_FILE=/cacert.pem

CMD [ "/digitalocean-dyn-domain" ]

