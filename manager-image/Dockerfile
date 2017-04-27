FROM ubuntu:16.04
RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev libssl1.0.0 && rm -rf /var/lib/apt/lists/*

COPY traefik-manager-bin /
ENTRYPOINT ["/traefik-manager-bin"]
#--config /sozu.toml --api 'http://127.0.0.1:8080/api'"]
