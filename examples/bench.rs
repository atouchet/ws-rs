/// A simple, but immature, benchmark client for destroying other WebSocket frameworks and proving
/// WS-RS's performance excellence. ;)
/// Make sure you allow for enough connections in your OS (e.g. ulimit -Sn 10000).

extern crate ws;
extern crate url;
extern crate clock_ticks;
extern crate env_logger;

// Try this against node for some fun

// TODO: Separate this example into a separate lib
// TODO: num threads, num connections per thread, num concurrent connections per thread, num
// messages per connection, length of message, text or binary

use ws::{WebSocket, Sender, CloseCode, Handler, Message, Handshake, Result};

const CONNECTIONS: usize = 10_000; // simultaneous
const MESSAGES: usize = 10;
static MESSAGE: &'static str = "TEST TEST TEST TEST TEST TEST TEST TEST";

fn main () {
    env_logger::init().unwrap();

    let url = url::Url::parse("ws://127.0.0.1:3012").unwrap();

    struct Connection {
        out: Sender,
        count: usize,
        time: u64,
        total: u64,
    }

    impl Handler for Connection {

        fn on_open(&mut self, _: Handshake) -> Result<()> {
            try!(self.out.send(MESSAGE));
            self.count += 1;
            Ok(self.time = clock_ticks::precise_time_ms())
        }

        fn on_message(&mut self, msg: Message) -> Result<()> {
            assert_eq!(msg.as_text().unwrap(), MESSAGE);
            if self.count > MESSAGES {
                try!(self.out.close(CloseCode::Normal));
            } else {
                try!(self.out.send(MESSAGE));
                let time = clock_ticks::precise_time_ms();
                // println!("time {}", time -self.time);
                self.total += time - self.time;
                self.count += 1;
                self.time = time;
            }
            Ok(())
        }
    }

    let mut ws = WebSocket::new(|out| {
        Connection {
            out: out,
            count: 0,
            time: 0,
            total: 0,
        }
    }).unwrap();


    for _ in 0..CONNECTIONS {
        ws.connect(url.clone()).unwrap();
    }
    let start = clock_ticks::precise_time_ms();
    ws.run().unwrap();
    println!("Total time. {}", clock_ticks::precise_time_ms() - start)
}
