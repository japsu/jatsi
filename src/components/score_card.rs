use crate::game::Game;

use dioxus::prelude::*;
#[derive(Props)]
pub struct ScoreCardProps<'a> {
  pub game: &'a UseRef<Game>,
}

#[allow(non_snake_case)]
pub fn ScoreCard<'a>(cx: Scope<'a, ScoreCardProps<'a>>) -> Element {
  let game = cx.props.game.read();

  let player_headers = game
    .players
    .iter()
    .map(|player| rsx! ( th { [player.name.clone()] }));

  let scoring_rows = game
    .ruleset
    .scorings
    .iter()
    .enumerate()
    .map(|(i, scoring)| {
      let player_scorings = game.players.iter().map(move |player| {
        if let Some(points) = player.score_sheet[i] {
          rsx!(td { [points.to_string()] })
        } else {
          rsx!(td {})
        }
      });

      rsx! (
        tr {
          th { [scoring.name()] }
          player_scorings
        }
      )
    });

  let total_footers = game
    .players
    .iter()
    .map(|player| {
      player
        .score_sheet
        .iter()
        .map(|maybe_points| maybe_points.unwrap_or(0))
        .sum::<u64>()
    })
    .map(|points| rsx!(th { [points.to_string()] }));

  cx.render(rsx! (
    table {
      thead {
        tr {
          th {}
          player_headers
        }
      }

      tbody {
        scoring_rows
      }

      tfoot {
        tr {
          th { "Total" }
          total_footers
        }
      }
    }
  ))
}
