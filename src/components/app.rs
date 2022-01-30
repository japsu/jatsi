use crate::components::die::Die;
use crate::components::score_card::ScoreCard;
use crate::game::Game;
use yew::{html, Component, Context, Html};

pub enum Msg {}

pub struct App {
  game: Game,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      game: Game::dummy(),
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    false
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <>
        <ScoreCard scorings={self.game.ruleset.scorings.clone()} players={self.game.players.clone()} />
        <div class="dice">
          <Die value={2} />
          <Die value={3} />
          <Die value={4} />
          <Die value={5} />
          <Die value={6} />
        </div>
        <div class="container">
          <button>{"Toss a die for your witcher!"}</button>
        </div>
      </>
    }
  }
}
