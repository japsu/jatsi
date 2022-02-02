use yew::{function_component, html, Properties};

use crate::game::Game;

use std::rc::Rc;

#[derive(Properties)]
pub struct ScoreCardProps {
  pub game: Rc<Game>,
}

impl PartialEq for ScoreCardProps {
  fn eq(&self, other: &Self) -> bool {
    Rc::ptr_eq(&self.game, &other.game)
  }
}

#[function_component(ScoreCard)]
pub fn score_table(props: &ScoreCardProps) -> Html {
  let player_headers = props
    .game
    .players
    .iter()
    .map(|player| html! { <th>{player.name.clone() }</th> });

  let scoring_rows = props
    .game
    .ruleset
    .scorings
    .iter()
    .enumerate()
    .map(|(i, scoring)| {
      let player_scorings = props.game.players.iter().map(|player| {
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

  let total_footers = props
    .game
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
