use crate::components::die::Die;
use crate::components::score_card::ScoreCard;
use crate::dice::roll_dice_keeping;
use crate::game::Game;
use itertools::izip;
use std::rc::Rc;
use yew::{events::MouseEvent, html, Component, Context, Html};

pub enum Msg {
  ToggleHold(usize),
  Reroll,
}

use Msg::*;

pub struct App {
  game: Rc<Game>,
  roll: Vec<u64>,
  keep: Vec<bool>,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    let game = Game::dummy();
    let num_dice = game.ruleset.dice.len();
    let roll = vec![1; num_dice];
    let keep = vec![false; num_dice];

    Self {
      game: Rc::new(game),
      roll,
      keep,
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    let num_dice = self.game.ruleset.dice.len();

    match msg {
      ToggleHold(i) => self.keep[i] = !self.keep[i],
      Reroll => {
        self.roll = roll_dice_keeping(&self.game.ruleset.dice, &self.roll, &self.keep);
      }
    }

    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let dice = izip!(&self.roll, &self.keep)
      .enumerate()
      .map(|(ind, (&value, &kept))| {
        let onclick = ctx.link().callback(move |e: MouseEvent| {
          e.prevent_default();
          ToggleHold(ind)
        });
        html! { <Die value={value} onclick={onclick} keep={kept} />}
      });

    let roll = ctx.link().callback(|_| Reroll);

    html! {
      <>
        <ScoreCard game={self.game.clone()} />
        <div class="dice">
          { for dice }
        </div>
        <div class="container">
          <button onclick={roll}>{"Toss a die for your witcher!"}</button>
        </div>
      </>
    }
  }
}
