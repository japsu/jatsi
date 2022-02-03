mod components;
mod dice;
mod game;
mod rules;

use crate::components::app::App;

use dioxus::prelude::*;

fn main() {
  dioxus::web::launch(App);
}
