#!/bin/sh

../../traefik-manager-bin/target/debug/traefik-manager-bin --config ./config.toml --api 'http://traefik-ui.local:1234/api'
