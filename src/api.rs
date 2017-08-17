use std::time::Duration;
use std::sync::mpsc;

use futures::Future;
use futures::future;
use futures::stream::Stream;
use serde_json;
use tokio_timer;
use tokio_core;
use hyper;

use sozu_command::channel::Channel;
use sozu_command::messages::Order;
use sozu_command::state::ConfigState;


use hyper::Client;
use providers;


pub fn driver(url: hyper::Url, tx: mpsc::Sender<Order>) {

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::new(&handle);
    let d: Duration = Duration::from_millis(1000);

    let mut state = ConfigState::new();
    state.add_http_address("127.0.0.1".to_string(), 80);

    let work = tokio_timer::Timer::default().interval(d).fold(state, move |state, ()| {
      let tx = tx.clone();
      let st = state.clone();
      client.get(url.clone()).map_err(|_| tokio_timer::TimerError::TooLong).and_then(|res| {
        //println!("Response: {}", res.status());
        //println!("Headers: \n{}", res.headers());

        res.body().collect().map(|buffers| {
          let mut v = Vec::new();
          for buf in buffers {
            v.extend(buf.as_ref());
          }
          v
        }).and_then(|buffer| {
          let p: serde_json::Result<providers::Providers> = serde_json::from_slice(&buffer);
          //println!("parsed providers list:\n{:?}", p);
          match p {
            Ok(providers) => future::ok(providers),
            Err(_)        => future::err(hyper::Error::Status),
          }
        }).map(move |providers| {
          let new_state = providers.to_http_state("http", "127.0.0.1", 80);
          //println!("parsed state: {:?}", new_state);

          //println!("\nDIFF:\n");
          for order in state.diff(&new_state) {
            println!("order: {:?}", order);
            tx.send(order.clone());
          }

          new_state
        })
        .map_err(|_| tokio_timer::TimerError::TooLong)
      }).or_else(move |error|{
        println!("got error: {:?}", error);
        Ok(st)
      })
    });

    core.run(work).unwrap();
}
