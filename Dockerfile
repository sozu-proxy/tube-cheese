FROM alpine:edge as builder

COPY . /source/

RUN apk add --no-cache --virtual .build-dependencies \
  cargo \
  g++ \
  gcc \
  musl-dev \
  rust
RUN apk add --no-cache openssl-dev
WORKDIR /source
RUN cargo build --release


#CMD ["/bin/sozu", "--help"]


FROM alpine:edge
RUN apk add --no-cache openssl
COPY --from=builder /source/target/release/tube-cheese /bin/tube-cheese
CMD ["/bin/tube-cheese", "--help"]
