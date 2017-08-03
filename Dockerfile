FROM alpine:edge

COPY . /source/

RUN apk add --no-cache --virtual .build-dependencies \
  cargo \
  g++ \
  gcc \
  musl-dev \
  rust && \
  apk add --no-cache openssl-dev && \
  cd /source && \
  cargo build --release && \
  echo "plop"
#  cp /source/target/release/sozu /bin/sozu && \
#  cp /source/target/release/sozuctl /bin/sozuctl && \
#  cd / && \
#  apk del .build-dependencies && \
#  apk del && \
#  rm -rf /source && \
#  rm -rf /root/.cargo


#CMD ["/bin/sozu", "--help"]
