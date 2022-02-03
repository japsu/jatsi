use crate::components::die::Die;
use crate::components::score_card::ScoreCard;
use crate::dice::roll_dice_keeping;
use crate::game::Game;
use dioxus::events::MouseEvent;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
  let game = use_ref(&cx, || Game::dummy());
  let num_dice = game.read().ruleset.dice.len();
  let keep = use_ref(&cx, || vec![false; num_dice]);

  let dice = game
    .read()
    .roll
    .iter()
    .zip(keep.read().iter())
    .enumerate()
    .map(|(ind, (&value, &kept))| {
      rsx!(Die {
        value: value,
        keep: kept,
        onclick: move |_: MouseEvent| {
          let mut keep = keep.write();
          keep[ind] = !keep[ind];
        },
      })
    })
    .collect::<Vec<LazyNodes>>();

  let toss = move |_| {
    let mut game = game.write();
    let keep = keep.read();
    game.roll = roll_dice_keeping(&game.ruleset.dice, &game.roll, &keep);
  };

  rsx!(cx,
    div {
      ScoreCard {
        game: &game
      }

      div {
        class: "dice",

        dice
      }

      div {
        class: "container",

        button {
          onclick: toss,
          prevent_default: "onclick",
          "Toss a die for your witcher!"
        }
      }
    }
  )
}
