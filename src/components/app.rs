use crate::components::die::Die;
use crate::components::score_card::ScoreCard;
use crate::dice::roll_dice_keeping;
use crate::game::Game;
use itertools::izip;
use std::{cell::RefCell, rc::Rc};
use yew::{events::MouseEvent, html, Component, Context, Html};

pub enum Msg {
  ToggleHold(usize),
  Reroll,
}

use Msg::*;

pub struct App {
  game: Rc<RefCell<Game>>,
  keep: Vec<bool>,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    let game = Game::dummy();
    let num_dice = game.ruleset.dice.len();
    let keep = vec![false; num_dice];

    Self {
      game: Rc::new(RefCell::new(game)),
      keep,
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    let mut game = self.game.borrow_mut();

    match msg {
      ToggleHold(i) => self.keep[i] = !self.keep[i],
      Reroll => {
        game.roll = roll_dice_keeping(&game.ruleset.dice, &game.roll, &self.keep);
      }
    }

    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let game = self.game.borrow();
    let dice = izip!(&game.roll, &self.keep)
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
