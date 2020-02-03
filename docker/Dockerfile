## Set up the project and build it for release

FROM ekidd/rust-musl-builder:1.41.0
RUN git clone 'https://github.com/spikegrobstein/digitalocean-dyn-domain.git'
WORKDIR /home/rust/src/digitalocean-dyn-domain
RUN cargo build --release

## Build the actual container

FROM scratch

COPY --from=0 /home/rust/src/digitalocean-dyn-domain/target/x86_64-unknown-linux-musl/release/digitalocean-dyn-domain /digitalocean-dyn-domain

# Add the cacerts file so that we can make https calls from our program
COPY files/cacert.pem /cacert.pem
ENV SSL_CERT_FILE=/cacert.pem

CMD [ "/digitalocean-dyn-domain" ]

