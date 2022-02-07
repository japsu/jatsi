use either::Either::{self, Left, Right};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

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

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerMessage {
  JoinGame(String),
  StartGame,
  ToggleHold(usize),
  Roll,
  Place(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum GameMessage {
  PlayerMessage(usize, PlayerMessage),
  PlayerTurn(usize),
  RollResult(Vec<u64>),
  GameFinished,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InvalidAction {
  NotYourTurn,
  WrongState,
  RollsExceeded,
  OutOfBounds,
}

impl Display for InvalidAction {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match &self {
      Self::NotYourTurn => write!(f, "not your turn"),
      Self::WrongState => write!(f, "cannot perform this action in this state"),
      Self::RollsExceeded => write!(f, "rerolls exceeded"),
      Self::OutOfBounds => write!(f, "out of bounds (this should'nt happen :)"),
    }
  }
}

impl Error for InvalidAction {}

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
      state: State::Start,
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
  ) -> Result<Vec<GameMessage>, InvalidAction> {
    if from_player != self.player_in_turn {
      if matches!(self.state, State::Start) && matches!(msg, PlayerMessage::JoinGame(..)) {
        // Joining the game in the Start state does not care about turn order
      } else {
        // All other messages do
        return Err(InvalidAction::NotYourTurn);
      }
    }

    let mut messages = Vec::with_capacity(2);
    messages.push(GameMessage::PlayerMessage(from_player, msg.clone()));

    match msg {
      PlayerMessage::JoinGame(name) => {
        let name = name.clone();
        let num_rows = self.ruleset.scorings.len();
        let score_sheet = vec![None; num_rows];
        self.players.push(Player { name, score_sheet });
      }
      PlayerMessage::StartGame => {
        if let State::Start = &self.state {
          self.round = 0;
          self.player_in_turn = 0;
          self.times_rolled = 0;
          messages.push(GameMessage::PlayerTurn(self.player_in_turn))
        }
      }
      _ => todo!(),
    }

    self.message_history.append(&mut messages.clone());

    Ok(messages)
  }

  pub fn dummy() -> Self {
    let rules = ee_rules();
    let mut game = Self::new(rules);
    game.handle_player_message(0, &PlayerMessage::JoinGame("Japsu".into()));
    game
  }

  fn roll_dice(&self) -> Vec<u64> {
    roll_dice(&self.ruleset.dice)
  }

  fn roll_dice_keeping(&self, keep: &[bool]) -> Vec<u64> {
    roll_dice_keeping(&self.ruleset.dice, &self.roll, keep)
  }

  pub fn start(&mut self) {
    let roll = roll_dice(&self.ruleset.dice);
    self._start(roll)
  }

  fn _start(&mut self, roll: Vec<u64>) {
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
      self.state = State::Reroll;
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
      self.state = State::Reroll;
    } else {
      // End of round
      self.player_in_turn = 0;
      self.round += 1;

      if self.round <= self.ruleset.rounds() {
        self.state = State::Reroll;
        self.times_rolled = 1;
      } else {
        self.state = State::End;
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
  use std::collections::VecDeque;

  use super::*;
  use crate::rules::{ee_rules, Scoring};

  /// Runs a game and compares its messages to an expected transcript.
  /// All PlayerMessages are sent to the Game, and following non-PlayerMessage GameMessages are checked to be equal.
  fn run_game(game: &mut Game, expected_messages: Vec<GameMessage>) -> Result<(), Box<dyn Error>> {
    let mut grouped_messages: Vec<Vec<GameMessage>> = vec![];

    // Group messages so that there is always a PlayerMessage and then server messages followed by it
    {
      let mut player_message: Option<GameMessage> = None;
      let mut response_messages: Vec<GameMessage> = vec![];

      for message in expected_messages {
        match message {
          GameMessage::PlayerMessage(..) => {
            if let Some(previous_player_message) = player_message {
              let mut group = vec![previous_player_message];
              group.append(&mut response_messages);
              grouped_messages.push(group);
              player_message = Some(message);
            } else {
              player_message = Some(message);
            }
          }
          _ => response_messages.push(message),
        }
      }

      if let Some(previous_player_message) = player_message {
        let mut group = vec![previous_player_message];
        group.append(&mut response_messages);
        grouped_messages.push(group);
      }
    }

    for group in grouped_messages {
      let player_msg = group.first();
      if let Some(GameMessage::PlayerMessage(from_player, player_message)) = player_msg {
        let actual_messages = game.handle_player_message(*from_player, player_message)?;
        assert_eq!(actual_messages, group);
      } else {
        panic!(
          "First message of group should have been a PlayerMessage but was {:?}",
          player_msg
        )
      }
    }

    Ok(())
  }

  #[test]
  fn test_scoreboard() {
    let game = Game::dummy();
    let scoreboard = game.scoreboard();
    assert_eq!(scoreboard.len(), 1);
    let (score, name) = scoreboard[0];
    assert_eq!(score, 0);
    assert_eq!(name, "Japsu");
  }

  #[test]
  fn test_reroll_place() -> Result<(), Box<dyn Error>> {
    let ruleset = ee_rules();
    let mut game = Game::new(ruleset);

    run_game(
      &mut game,
      vec![
        GameMessage::PlayerMessage(0, PlayerMessage::JoinGame("Japsu".into())),
        GameMessage::PlayerMessage(0, PlayerMessage::StartGame),
        GameMessage::PlayerTurn(0),
      ],
    )?;

    // TODO continue here

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

    Ok(())
  }
}
