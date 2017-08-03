FROM alpine:edge as builder

COPY . /source/

RUN apk add --no-cache --virtual .build-dependencies \
  cargo \
  g++ \
  gcc \
  musl-dev \
  rust && \
RUN apk add --no-cache openssl-dev
RUN cd /source
RUN cargo build --release 


#CMD ["/bin/sozu", "--help"]


FROM alpine:edge
RUN apk add --no-cache openssl-dev
WORKDIR /root/
COPY --from=builder /source/target/release/sozu /bin/sozu
CMD ["./app"]
