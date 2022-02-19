use crate::die::Die;
use crate::score_card::ScoreCard;
use dioxus::events::MouseEvent;
use dioxus::prelude::*;
use dioxus_websocket_hooks::use_ws_context_provider;
use jatsi_shared::dice::roll_dice_keeping;
use jatsi_shared::game::Game;

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
  let game = use_ref(&cx, || Game::dummy());
  let num_dice = game.read().ruleset.dice.len();
  let keep = use_ref(&cx, || vec![false; num_dice]);

  use_ws_context_provider(&cx, "ws://localhost:8088", move |msg| {
    println!("{:?}", msg);
  });

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
