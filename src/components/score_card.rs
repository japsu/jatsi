use yew::{function_component, html, Properties};

use crate::game::Game;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Properties)]
pub struct ScoreCardProps {
  pub game: Rc<RefCell<Game>>,
}

impl PartialEq for ScoreCardProps {
  fn eq(&self, other: &Self) -> bool {
    *self.game.borrow() == *other.game.borrow()
  }
}

#[function_component(ScoreCard)]
pub fn score_table(props: &ScoreCardProps) -> Html {
  let game = props.game.borrow();
  let player_headers = game
    .players
    .iter()
    .map(|player| html! { <th>{player.name.clone() }</th> });

  let scoring_rows = game
    .ruleset
    .scorings
    .iter()
    .enumerate()
    .map(|(i, scoring)| {
      let player_scorings = game.players.iter().map(|player| {
        if let Some(points) = player.score_sheet[i] {
          html! { <td>{points.to_string()}</td> }
        } else {
          html! { <td></td> }
        }
      });

      html! {
        <tr>
          <th>{scoring.name()}</th>
          { for player_scorings }
        </tr>
      }
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
    .map(|points| html! { <th>{points.to_string()}</th> });

  html! {
    <table>
      <thead>
        <tr>
          <th/>
          { for player_headers }
        </tr>
      </thead>
      <tbody>
        { for scoring_rows }
      </tbody>
      <tfoot>
        <tr>
          <th>{"Total"}</th>
          { for total_footers }
        </tr>
      </tfoot>
    </table>
  }
}
