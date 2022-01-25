use either::Either::{self, Left, Right};
use itertools::izip;
use rand::Rng;

use crate::rules::{update_score_sheet, InvalidUpdate, Ruleset};

pub struct Player {
  pub name: String,
  pub score_sheet: Vec<Option<u64>>,
}

pub struct Start {}
pub struct Reroll {
  pub round: usize,
  pub player_in_turn: usize,
  pub times_rolled: u64,
  pub roll: Vec<u64>,
}

pub struct Place {
  pub round: usize,
  pub player_in_turn: usize,
  pub roll: Vec<u64>,
}

pub struct End {}

pub struct Game<S> {
  pub players: Vec<Player>,
  pub ruleset: Ruleset,
  pub state: S,
}

fn roll_dice(dice: &[u64]) -> Vec<u64> {
  let mut rng = rand::thread_rng();
  dice.iter().map(|&sides| rng.gen_range(1..=sides)).collect()
}

impl Game<Start> {
  pub fn new(ruleset: Ruleset) -> Game<Start> {
    Game {
      players: Vec::new(),
      ruleset,
      state: Start {},
    }
  }

  pub fn add_player(&mut self, name: String) {
    let num_rows = self.ruleset.scorings.len();
    let score_sheet = vec![None; num_rows];
    self.players.push(Player { name, score_sheet });
  }

  pub fn start(self) -> Game<Reroll> {
    let roll = roll_dice(&self.ruleset.dice);
    self._start(roll)
  }

  fn _start(self, roll: Vec<u64>) -> Game<Reroll> {
    Game {
      players: self.players,
      ruleset: self.ruleset,
      state: Reroll {
        round: 0,
        player_in_turn: 0,
        times_rolled: 1,
        roll,
      },
    }
  }
}

/// Common implementation of Game<Reroll>::place and Game<Place>::place.
fn _place(
  mut players: Vec<Player>,
  ruleset: Ruleset,
  round: usize,
  player_in_turn: usize,
  selected_row: usize,
  roll: &[u64],
  next_roll: Vec<u64>,
) -> Result<Either<Game<Reroll>, Game<End>>, InvalidUpdate> {
  let score_sheet = update_score_sheet(
    &players[player_in_turn].score_sheet,
    &ruleset.scorings,
    selected_row,
    &roll,
  )?;

  players[player_in_turn].score_sheet = score_sheet;

  let player_in_turn = player_in_turn + 1;
  if player_in_turn < players.len() {
    Ok(Left(Game {
      ruleset,
      players,
      state: Reroll {
        round,
        player_in_turn,
        times_rolled: 1,
        roll: next_roll,
      },
    }))
  } else {
    // End of round
    let player_in_turn = 0;
    let round = round + 1;

    if round <= ruleset.rounds() {
      Ok(Left(Game {
        ruleset,
        players,
        state: Reroll {
          round,
          player_in_turn,
          times_rolled: 1,
          roll: next_roll,
        },
      }))
    } else {
      Ok(Right(Game {
        ruleset,
        players,
        state: End {},
      }))
    }
  }
}

impl Game<Reroll> {
  pub fn reroll(self, keep: &[bool]) -> Either<Game<Reroll>, Game<Place>> {
    let mut rng = rand::thread_rng();

    let roll = izip!(&self.ruleset.dice, &self.state.roll, keep)
      .map(|(&sides, &old_value, &kept)| {
        if kept {
          old_value
        } else {
          rng.gen_range(1..=sides)
        }
      })
      .collect();

    self._reroll(roll)
  }

  fn _reroll(self, roll: Vec<u64>) -> Either<Game<Reroll>, Game<Place>> {
    let times_rolled = self.state.times_rolled + 1;
    let player_in_turn = self.state.player_in_turn;
    let round = self.state.round;

    if times_rolled < self.ruleset.rolls {
      // rerolls left, can reroll or place
      Left(Game {
        state: Reroll {
          round,
          player_in_turn,
          times_rolled,
          roll,
        },
        ..self
      })
    } else {
      // all rerolls used, can only place
      Right(Game {
        players: self.players,
        ruleset: self.ruleset,
        state: Place {
          round,
          player_in_turn,
          roll,
        },
      })
    }
  }

  fn place(self, selected_row: usize) -> Result<Either<Game<Reroll>, Game<End>>, InvalidUpdate> {
    let next_roll = roll_dice(&self.ruleset.dice);
    _place(
      self.players,
      self.ruleset,
      self.state.round,
      self.state.player_in_turn,
      selected_row,
      &self.state.roll,
      next_roll,
    )
  }

  fn _place(
    self,
    selected_row: usize,
    next_roll: Vec<u64>,
  ) -> Result<Either<Game<Reroll>, Game<End>>, InvalidUpdate> {
    _place(
      self.players,
      self.ruleset,
      self.state.round,
      self.state.player_in_turn,
      selected_row,
      &self.state.roll,
      next_roll,
    )
  }
}

impl Game<Place> {
  fn place(self, selected_row: usize) -> Result<Either<Game<Reroll>, Game<End>>, InvalidUpdate> {
    let next_roll = roll_dice(&self.ruleset.dice);
    _place(
      self.players,
      self.ruleset,
      self.state.round,
      self.state.player_in_turn,
      selected_row,
      &self.state.roll,
      next_roll,
    )
  }

  fn _place(
    self,
    selected_row: usize,
    next_roll: Vec<u64>,
  ) -> Result<Either<Game<Reroll>, Game<End>>, InvalidUpdate> {
    _place(
      self.players,
      self.ruleset,
      self.state.round,
      self.state.player_in_turn,
      selected_row,
      &self.state.roll,
      next_roll,
    )
  }
}

impl Game<End> {
  fn rematch(self) -> Game<Start> {
    Game {
      players: self.players,
      ruleset: self.ruleset,
      state: Start {},
    }
  }

  fn scoreboard(&self) -> Vec<(u64, &str)> {
    let mut result: Vec<(u64, &str)> = self
      .players
      .iter()
      .map(|player| {
        (
          player
            .score_sheet
            .iter()
            .map(|item| item.or(Some(0)).unwrap())
            .sum(),
          player.name.as_str(),
        )
      })
      .collect();

    result.sort();
    result.reverse();
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::rules::{ee_rules, Scoring};

  #[test]
  fn test_scoreboard() {
    let ruleset = ee_rules();
    let score_sheet = vec![None; ruleset.scorings.len()];
    let players = vec![Player {
      name: "Japsu".into(),
      score_sheet,
    }];
    let game = Game {
      players,
      ruleset,
      state: End {},
    };
    let scoreboard = game.scoreboard();
    assert_eq!(scoreboard.len(), 1);
    let (score, name) = scoreboard[0];
    assert_eq!(score, 0);
    assert_eq!(name, "Japsu");
  }

  #[test]
  fn test_reroll_place() {
    let ruleset = ee_rules();
    let mut game = Game::new(ruleset);
    game.add_player("Japsu".into());

    let game = game._start(vec![5, 5, 4, 3, 2]);

    // we still have rolls left
    let game = match game._reroll(vec![5, 5, 5, 1, 1]) {
      Left(game) => game,
      Right(_) => panic!(),
    };

    // okay now we have full house, let's place
    let full_house_index = game
      .ruleset
      .scorings
      .iter()
      .position(|scoring| {
        if let Scoring::FullHouse { .. } = scoring {
          true
        } else {
          false
        }
      })
      .unwrap();

    let _game = match game._place(full_house_index, vec![1, 1, 1, 1, 1]).unwrap() {
      Left(game) => game,
      Right(_) => panic!(),
    };
  }
}
