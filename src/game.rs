use either::Either::{self, Left, Right};
use itertools::izip;
use rand::Rng;

use crate::dice::{roll_dice, roll_dice_keeping};
use crate::rules::{ee_rules, update_score_sheet, InvalidUpdate, Ruleset};

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
  pub name: String,
  pub score_sheet: Vec<Option<u64>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum State {
  Start,
  Reroll,
  Place,
  End,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerMessage {
  StartGame,
  ToggleHold(usize),
  Roll,
  Place(usize),
}

pub enum GameMessage {
  PlayerJoined(usize, String),
  PlayerMessage(usize, PlayerMessage),
  RollResult(Vec<u64>),
  GameFinished,
}

pub enum InvalidAction {
  NotYourTurn,
  WrongState,
  RollsExceeded,
  OutOfBounds,
}

pub struct Game {
  pub message_history: Vec<GameMessage>,
  pub players: Vec<Player>,
  pub ruleset: Ruleset,
  pub state: State,
  pub round: usize,
  pub player_in_turn: usize,
  pub times_rolled: u64,
  pub roll: Vec<u64>,
}

impl Game {
  pub fn new(ruleset: Ruleset) -> Self {
    let num_dice = ruleset.dice.len();
    Self {
      message_history: Vec::new(),
      players: Vec::new(),
      ruleset,
      state: Start {},
      round: 0,
      player_in_turn: 0,
      times_rolled: 0,
      roll: vec![1; num_dice],
    }
  }

  pub fn handle_player_message(
    &mut self,
    from_player: usize,
    msg: &PlayerMessage,
  ) -> Result<GameMessage, InvalidAction> {
    if from_player != self.player_in_turn {
      return Err(InvalidAction::NotYourTurn);
    }

    match msg {
      PlayerMessage::StartGame => {
        if let State::Start = &self.state {
          self.start();
        }
      }
      _ => todo!(),
    }

    Ok(GameMessage::PlayerMessage(self.player_in_turn, *msg))
  }
}

impl Game {
  pub fn dummy() -> Self {
    let rules = ee_rules();
    let mut game = Self::new(rules);
    game.add_player("Japsu".into());
    game
  }

  fn roll_dice(&self) -> Vec<u64> {
    roll_dice(&self.ruleset.dice)
  }

  fn roll_dice_keeping(&self, keep: &[bool]) -> Vec<u64> {
    roll_dice_keeping(&self.ruleset.dice, &self.roll, keep)
  }

  // TODO state checking
  pub fn add_player(&mut self, name: String) {
    let num_rows = self.ruleset.scorings.len();
    let score_sheet = vec![None; num_rows];
    self.players.push(Player { name, score_sheet });
  }

  pub fn start(&mut self) {
    let roll = roll_dice(&self.ruleset.dice);
    self._start(roll)
  }

  fn _start(&mut self, roll: Vec<u64>) {
    self.round = 0;
    self.player_in_turn = 0;
    self.times_rolled = 1;
    self.roll = self.roll_dice();
  }

  pub fn reroll(&mut self, keep: &[bool]) {
    let roll = self.roll_dice_keeping(keep);
    self._reroll(roll)
  }

  fn _reroll(&mut self, roll: Vec<u64>) {
    let times_rolled = self.times_rolled + 1;
    let player_in_turn = self.player_in_turn;
    let round = self.round;

    if times_rolled >= self.ruleset.rolls {
      self.state = Reroll;
    }
  }

  fn place(&mut self, selected_row: usize) {
    let next_roll = self.roll_dice();
    self._place(selected_row, next_roll)
  }

  fn _place(&mut self, selected_row: usize, next_roll: Vec<u64>) {
    let score_sheet = update_score_sheet(
      &self.players[self.player_in_turn].score_sheet,
      &self.ruleset.scorings,
      selected_row,
      &self.roll,
    )
    .unwrap();

    self.players[self.player_in_turn].score_sheet = score_sheet;

    self.player_in_turn += 1;
    if self.player_in_turn < self.players.len() {
      self.times_rolled = 1;
      self.state = Reroll;
    } else {
      // End of round
      self.player_in_turn = 0;
      self.round += 1;

      if self.round <= self.ruleset.rounds() {
        self.state = Reroll;
        self.times_rolled = 1;
      } else {
        self.state = End;
      }
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
      player_in_turn: 0,
      roll: vec![],
      round: 0,
      times_rolled: 0,
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

    game._start(vec![5, 5, 4, 3, 2]);

    // bah! puny small straight! let's keep the 5 5 and reroll the 4 3 2

    // we still have rolls left
    game._reroll(vec![5, 5, 5, 1, 1]);

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

    game._place(full_house_index, vec![1, 1, 1, 1, 1]);
  }
}
