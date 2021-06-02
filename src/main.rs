//! An introduction to fundamental `Router` and `Router Builder` concepts to create a routing tree.
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;

const HELLO_ROUTER: &str = "Hello Router!";

/// Create a `Handler` that is invoked for requests to the path "/"
pub fn say_hello(state: State) -> (State, &'static str) {
  (state, HELLO_ROUTER)
}

/// Create a `Router`
///
/// Provides tree of routes with only a singel top level entry that looks like:
///
/// /                           --> GET
///
/// If no match for a request is found a 404 will be returned. Both the HTTP verb and the request
/// path are considered when determining if the request matches a defined route.
fn router() -> Router {
  build_simple_router(|route| {
    // For the path "/" invoke the handler "say_hello"
    route.get("/").to(say_hello);
  })
}

/// Start a server and use a `Router` to dispatch requests
pub fn main() {
  let addr = "127.0.0.1:8080";
  println!("Listening for requests at http://{}", addr);

  // All incoming requests are delegated to the router for further analysis and dispatch
  gotham::start(addr, router())
}
