# tube-cheese, a configuration manager for sozu-proxy using traefik

Since writing clients for every configuration service is a bit time consuming,
for testing purposes, we built a tool that reuses all of the clients built in
traefik, the Go reverse proxy.

The way it works:

- launch traefik
- tell traefik to expose its API, but listen on something else than 80 or 8080
- launch sozu
- launch tube-cheese, pointing it to sozu's command socket, and traefik's API
- sozu is now getting configuration information from traefik

It required some tweaks to the approach, as traefik replaces completely the old
conf with the new one on each change, whereas sozu receives configuration diffs.

There is already a working example to use sozu as a kubernetes ingress.

## testing

Have a look at https://github.com/sozu-proxy/sozu-demo the demo project
