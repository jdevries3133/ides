FROM scratch
COPY ./target/x86_64-unknown-linux-musl/release/ides ides
ENTRYPOINT [ "./ides" ]
