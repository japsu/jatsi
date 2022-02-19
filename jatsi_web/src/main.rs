mod app;
mod die;
mod score_card;

use crate::app::App;

fn main() {
  dioxus::web::launch(App);
}
