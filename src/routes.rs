//! Holds routes and the helper methods to put pages together.

use gotham::{
  helpers::http::response::create_response,
  hyper::{Body, Response, StatusCode},
  state::*,
};
use rust_tags::{
  self, attributes, attributes::*, core::Fragment, tags, tags::*,
};

use super::state::User;

/// A simple function that renders the head given a title `t`.
/// Includes bootstrap and viewport tags.
fn header(t: &str) -> Fragment {
  head(&[
       tags::title(&[t.into()]),
       meta(&[charset("utf-8")]),
       link(&[href("https://cdn.jsdelivr.net/npm/bootstrap@5.0.1/dist/css/bootstrap.min.css"), rel("stylesheet")]),
       link(&[href("style.css"), rel("stylesheet")]),
       meta(&[name("viewport"), content("width=device-width, initial-scale=1")]),
  ])
}

/// Wraps the content in a container using bootstrap
fn container(content: &[Fragment]) -> Fragment {
  let mut c = vec![class("container")];
  c.extend_from_slice(content);

  body(&[class("text-center"), div(&c)])
}

/// The ending script tag for bootstrap
fn end() -> Fragment {
  script(&[src(
    "https://cdn.jsdelivr.net/npm/bootstrap@5.0.1/dist/js/bootstrap.min.js",
  )])
}

/// The actual routes
/// --------------------------------------------
/// A simple hello world route
pub fn hello_world(state: State) -> (State, Response<Body>) {
  let frag = html(&[
    header("Hello World!"),
    container(&[
      "Jim Hessin".into(),
      hr(&[]),
      a(&[href("#"), "My Blog <hello world />".into()]),
      br(),
      br(),
    ]),
    end(),
  ]);
  let res = create_response(&state, StatusCode::OK, mime::TEXT_HTML, frag.data);
  (state, res)
}

/// A basic form that greets the user if there is one and allows fields to be filled to set the
/// user.
pub fn greeting(state: State) -> (State, Response<Body>) {
  let (message, submit_text) = if let Some(user) = state.try_borrow::<User>() {
    (format!("Hello {} welcome to my page.", user.name), "Update")
  } else {
    (
      String::from("Welcome to my page, please enter your info below."),
      "Submit",
    )
  };

  let frag = html(&[
    header("User Info"),
    container(&[div(&[
      class("form-signin"),
      tags::form(&[
        h1(&[class("h3 mb-3 fw-normal"), message.into()]),
        div(&[
          class("form-floating"),
          input(&[
            class("form-control"),
            attributes::id("floatingEmail"),
            _type("email"),
            name("email"),
            placeholder("name@example.com"),
          ]),
          tags::label(&[
            attributes::for_("floatingEmail"),
            "Email address".into(),
          ]),
        ]),
        div(&[
          class("form-floating"),
          input(&[
            class("form-control"),
            attributes::id("floatingName"),
            _type("text"),
            name("name"),
            placeholder("John Doe"),
          ]),
          tags::label(&[attributes::for_("floatingName"), "Name".into()]),
        ]),
        input(&[
          class("w-100 btn btn-lg btn-primary"),
          _type("submit"),
          value(submit_text.into()),
        ]),
      ]),
    ])]),
  ]);

  let res = create_response(&state, StatusCode::OK, mime::TEXT_HTML, frag.data);
  (state, res)
}
