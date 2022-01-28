mod components;
mod game;
mod rules;

use crate::components::app::App;

fn main() {
    yew::start_app::<App>();
}
