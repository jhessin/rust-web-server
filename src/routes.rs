//! Holds routes and the helper methods to put pages together.

use gotham::state::*;
use rust_tags::{attributes::*, core::Fragment, tags, tags::*};

use super::state::User;

/// A simple function that renders the head given a title `t`.
/// Includes bootstrap and viewport tags.
fn header(t: &str) -> Fragment {
  head(&[
       tags::title(&[t.into()]),
       meta(&[charset("utf-8")]),
       link(&[href("https://cdn.jsdelivr.net/npm/bootstrap@5.0.1/dist/css/bootstrap.min.css"), rel("stylesheet")]),
       meta(&[name("viewport"), content("width=device-width, initial-scale=1")]),
  ])
}

/// Wraps the content in a container using bootstrap
fn container(content: &[Fragment]) -> Fragment {
  let mut c = vec![class("container")];
  c.extend_from_slice(content);

  body(&[div(&c)])
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
pub fn hello_world(state: State) -> (State, (mime::Mime, String)) {
  (
    state,
    (
      mime::TEXT_HTML,
      html(&[
        header("Hello World!"),
        container(&[
          "Jim Hessin".into(),
          hr(&[]),
          a(&[href("#"), "My Blog <hello world />".into()]),
          br(),
          br(),
        ]),
        end(),
      ])
      .data,
    ),
  )
}

/// A basic form that greets the user if there is one and allows fields to be filled to set the
/// user.
pub fn greeting(state: State) -> (State, (mime::Mime, String)) {
  if let Some(user) = state.try_borrow::<User>() {
    let message = String::from("Hello ") + &user.name + " welcome to my page.";
    (
      state,
      (
        mime::TEXT_HTML,
        html(&[
          header("User Info"),
          container(&[p(&[message.into()]), tags::form(&[])]),
        ])
        .data,
      ),
    )
  } else {
    (
      state,
      (
        mime::TEXT_HTML,
        html(&[
          header("User Info"),
          container(&[tags::form(&[
            input(&[_type("text"), name("email"), placeholder("Email")]),
            input(&[_type("text"), name("name"), placeholder("Name")]),
            input(&[_type("submit"), value("Update")]),
          ])]),
        ])
        .data,
      ),
    )
  }
}
