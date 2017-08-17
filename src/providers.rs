use std::collections::HashMap;
use sozu_command::state::{ConfigState};
use sozu_command::messages::{Order,HttpFront,Instance};
use url::Url;

#[derive(Clone, Debug, Deserialize)]
pub struct Providers(HashMap<String,Provider>);

#[derive(Clone, Debug, Deserialize)]
pub struct Provider {
  frontends: HashMap<String, Frontend>,
  backends:  HashMap<String, Backend>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Frontend {
  routes:       HashMap<String, Route>,
  backend:      String,
  entry_points: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Backend {
  load_balancer:   Option<Loadbalancer>,
  circuit_breaker: Option<CircuitBreaker>,
  servers:         HashMap<String, Server>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Route {
  rule: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Loadbalancer {
  method: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
  url: String,
  weight: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CircuitBreaker {
  expression: String,
}


impl Providers {
  pub fn to_http_state(&self, entry_point: &str, ip: &str, port: u16) -> ConfigState {
    //let mut proxy = HttpProxy::new(ip.to_string(), port);
    let mut proxy = ConfigState::new();
    proxy.add_http_address(ip.to_string(), port);

    for provider in self.0.values() {
      for (app_id, frontend) in provider.frontends.iter() {
        //FIXME: should check default entry points too
        if frontend.entry_points.as_ref().map(|entries| entries.contains(&entry_point.to_string())).unwrap_or(true) {
          if let Some(front) = make_front(&frontend.backend, &frontend.routes) {
            proxy.handle_order(&Order::AddHttpFront(front));
          }
        }
      }

      for (app_id, backend) in provider.backends.iter() {
        for (ref backend_name, ref server) in backend.servers.iter() {
          if let Ok(url) = Url::parse(&server.url) {
            let port = url.port().unwrap_or(80);
            if let Some(host) = url.host_str() {
                proxy.handle_order(&Order::AddInstance(Instance {
                  app_id: app_id.to_string(),
                  ip_address: host.to_string(),
                  port: port
                }));
              }
          }
        }
      }
    }

    proxy
  }

    /*
  pub fn to_tls_state(&self, entry_point: String, ip: String, port: u16) -> TlsProxy {
    let mut proxy = TlsProxy::new(ip, port);

    for provider in self.0.values() {
      for (app_id, frontend) in provider.frontends.iter() {
        //FIXME: should check default entry points too
        if frontend.entry_points.as_ref().map(|entries| entries.contains(&entry_point)).unwrap_or(true) {
          if let Some(front) = make_front(&frontend.backend, &frontend.routes) {
            proxy.handle_order(&Order::AddHttpFront(front));
          }
        }
      }

      for (app_id, backend) in provider.backends.iter() {
        for (ref backend_name, ref server) in backend.servers.iter() {
          if let Ok(url) = Url::parse(&server.url) {
            let port = url.port().unwrap_or(80);
            if let Some(host) = url.host_str() {
                proxy.handle_order(&Order::AddInstance(Instance {
                  app_id: app_id.to_string(),
                  ip_address: host.to_string(),
                  port: port
                }));
              }
          }
        }
      }
    }

    proxy
  }
  */
}

pub fn make_front(app_id: &str, routes: &HashMap<String, Route>) -> Option<HttpFront> {
  let mut hostname   = None;
  let mut path_begin = None;

  for route in routes.values() {
    if route.rule.starts_with("Host:") {
      hostname = Some((&route.rule[5..]).to_string());
      continue;
    }
    if route.rule.starts_with("PathPrefix:") {
      path_begin = Some((&route.rule[11..]).to_string());
      continue;
    }
  }

  hostname.map(|host| {
    HttpFront {
      app_id:     app_id.to_string(),
      hostname:   host,
      path_begin: path_begin.unwrap_or(String::from("/")),
    }
  })
}
