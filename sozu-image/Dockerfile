#FROM scratch
#FROM jimmycuadra/rust
FROM ubuntu:16.04

RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev libssl1.0.0 && rm -rf /var/lib/apt/lists/*
#RUN apt-get update && apt-get install -y supervisor

#ENV RUST_VERSION 1.16.0
#ENV CARGO_HOME /cargo
#ENV PATH $CARGO_HOME/bin:/root/.cargo/bin:$PATH
#ENV SRC_PATH /src

#RUN mkdir -p /var/log/supervisor
#COPY traefik /

# if sozu and traefik-manager-bin must be rebuilt
#RUN apt-get update && apt-get install -y supervisor
#
#RUN curl -sSf https://sh.rustup.rs \
#  | env -u CARGO_HOME sh -s -- -y --default-toolchain "$RUST_VERSION" \
#  && rustc --version && cargo --version \
#  && mkdir -p "$CARGO_HOME" "$SRC_PATH"
#RUN git clone --depth 1 https://github.com/sozu-proxy/sozu /sozu-source
#RUN cd /sozu-source/bin && cargo build && cp ../target/debug/sozu ../.. && cd ../..
#COPY traefik-manager.tar.gz /
#RUN tar xvf traefik-manager.tar.gz
#RUN cd traefik-manager && cargo build && cp target/debug/traefik-manager-bin .. && cd ..
COPY sozu /
#COPY traefik-manager-bin /
#COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf
#COPY traefik.toml /

COPY sozu.toml /
EXPOSE 80
EXPOSE 8080
#CMD ["/usr/bin/supervisord"]
ENTRYPOINT ["/sozu"]
#start -c /sozu.toml"]
