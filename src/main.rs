//! An introduction to fundamental `Router` and `Router Builder` concepts to create a routing tree.

extern crate gotham;
#[macro_use]
extern crate gotham_derive;

use futures::prelude::*;
use gotham::router::builder::*;
use gotham::router::Router;
use tokio::signal;

mod routes;
pub mod state;

/// Create a `Router`
///
/// Provides tree of routes with only a singel top level entry that looks like:
///
/// /                           --> GET
/// /hello                      --> GET
///
/// If no match for a request is found a 404 will be returned. Both the HTTP verb and the request
/// path are considered when determining if the request matches a defined route.
fn router() -> Router {
  build_simple_router(|route| {
    route.get("/").to(routes::greeting);
    route.get("/hello").to(routes::hello_world);
  })
}

/// Start a server and use a `Router` to dispatch requests
#[tokio::main]
pub async fn main() {
  let addr = "127.0.0.1:8080";
  let server = gotham::init_server(addr, || Ok(router()));

  // Future to wait for Ctrl+C.
  let signal = async {
    signal::ctrl_c().map_err(|_| ()).await?;
    println!("Ctrl+C pressed");
    Ok::<(), ()>(())
  };

  println!("Listening for requests at http://{}", addr);
  future::select(server.boxed(), signal.boxed()).await;
  println!("Shutting down gracefully");
}
