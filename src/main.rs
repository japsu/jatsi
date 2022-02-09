mod components;
mod dice;
mod errors;
mod game;
mod rules;

use crate::components::app::App;

fn main() {
  dioxus::web::launch(App);
}
