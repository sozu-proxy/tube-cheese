use std::thread;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use mio::*;
use mio::tcp::{TcpListener, TcpStream};
use mio_uds::UnixStream;
use sozu::channel::Channel;
use sozu_command::Order;
use sozu_command::data::{ConfigCommand,ConfigMessage,ConfigMessageAnswer};

pub fn driver(path: String) -> mpsc::Sender<Order> {
  let stream = UnixStream::connect(path).expect("could not connect to the command unix socket");
  let mut channel: Channel<ConfigMessage,ConfigMessageAnswer> = Channel::new(stream, 10000, 20000);
  channel.set_nonblocking(true);

  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let mut poll = Poll::new().unwrap();
    poll.register(&channel.sock, Token(0), Ready::all(), PollOpt::edge());

    // Create storage for events
    let mut events = Events::with_capacity(1024);
    let mut index  = 0u64;

    loop {
        poll.poll(&mut events, Some(Duration::from_millis(3000))).unwrap();

        for event in events.iter() {
            //println!("will handle event: {:?}", event);
            match event.token() {
                Token(0) => {
                    channel.handle_events(event.readiness());
                }
                _ => unreachable!(),
            }
        }

        //println!("will receive messages from proxy");
        loop {
          channel.run();
          let msg = channel.read_message();
          if msg.is_none() {
            //println!("message is none, breaking out of loop");
            break;
          }
          println!("reading message: {:?}", msg.unwrap());
        }

        //println!("will receive messages from kubernetes");
        loop {
          match rx.try_recv() {
            Err(mpsc::TryRecvError::Empty) => {
              //println!("channel empty");
              break;
            },
            Err(mpsc::TryRecvError::Disconnected) => {
              println!("channel disconnected");
              return;
            },
            Ok(msg) => {
              let sending_msg = ConfigMessage::new(
                format!("traefik-manager-{}", index),
                ConfigCommand::ProxyConfiguration(msg),
                Some("HTTP".to_string()),
                None
              );
              println!("sending {:?}", sending_msg);

              if !channel.write_message(&sending_msg) {
                println!("could not write message");
              } else {
                channel.run();
                //println!("sent message");
              }
              index += 1;
            }
          }
        }
    }

    println!("thread ended");
  });

  tx
}

