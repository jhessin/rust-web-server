//! A Hello world example application for working with Gotham

extern crate gotham;

use gotham::state::State;

const HELLO_WORLD: &str = "Hello World!";

/// Create a 'Handler' which is invoked when responding to a 'Request'.
pub fn say_hello(state: State) -> (State, &'static str) {
  (state, HELLO_WORLD)
}

/// Start a server and call the 'Handler' we've defined above for each 'Request' we receive.
pub fn main() {
  let addr = "127.0.0.1:8080";
  println!("Listening for requests at http://{}", addr);
  gotham::start(addr, || Ok(say_hello))
}

#[cfg(test)]
mod tests {
  use super::*;
  use gotham::{hyper::StatusCode, test::TestServer};

  #[test]
  fn receive_hello_world_response() {
    let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
    let response = test_server
      .client()
      .get("http://localhost")
      .perform()
      .unwrap();

    assert_eq!(response.status(), StatusCode::OK );

    let body = response.read_body().unwrap();
    assert_eq!(&body[..], b"Hello World!");
  }
}
