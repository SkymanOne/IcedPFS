FROM amd64/rust:1.61-alpine
RUN echo "Installation of required system packaged for the build..."
RUN apk update 
RUN apk add musl-dev openssl-dev sccache glib-dev
RUN rustup default nightly
RUN cargo install sccache
RUN cargo install fleet-rs
RUN echo "Installation of required system packaged for the build done!"
ENV RUSTC_WRAPPER=sccache