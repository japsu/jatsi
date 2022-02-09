use crate::dice::{roll_dice, roll_dice_keeping};
use crate::errors::InvalidAction;
use crate::rules::{ee_rules, update_score_sheet, Ruleset};

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
  pub name: String,
  pub score_sheet: Vec<Option<u64>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum State {
  Start,
  FirstRoll,
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
pub struct Game {
  pub message_history: Vec<GameMessage>,
  pub players: Vec<Player>,
  pub ruleset: Ruleset,
  pub state: State,
  pub round: usize,
  pub player_in_turn: usize,
  pub times_rolled: u64,
  pub roll: Vec<u64>,
  pub keep: Vec<bool>,
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
      keep: vec![false; num_dice],
    }
  }

  pub fn dummy() -> Self {
    let rules = ee_rules();
    let mut game = Self::new(rules);

    let messages = game
      .prepare(0, &PlayerMessage::JoinGame("Japsu".into()))
      .unwrap();

    for message in messages {
      game.commit(&message).unwrap();
    }

    game
  }

  /// Prepare takes a PlayerMessage and generates the GameMessages the game would respond with.
  /// However, no state is mutated here. Hence this method may be used to check if a particular action would succeed.
  /// Note: Unless you hold the authoritative Game, any RollResult returned will not be the final one.
  /// Call `commit` with each of the returned message to actually execute the actions.
  /// Called at the leader to process actions from players.
  /// TODO: `prepare` always returns one or two messages, out of which the first will always be `msg` wrapped into `PlayerMessage`.
  /// Need not necessarily use a Vec for that, could be a tuple.
  pub fn prepare(
    &self,
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
      PlayerMessage::JoinGame(..) => match &self.state {
        State::Start => {}
        _ => return Err(InvalidAction::WrongState),
      },
      PlayerMessage::StartGame => match &self.state {
        State::Start => messages.push(GameMessage::PlayerTurn(self.player_in_turn)),
        _ => return Err(InvalidAction::WrongState),
      },

      PlayerMessage::ToggleHold(num) => match &self.state {
        State::Reroll => {
          if *num >= self.ruleset.dice.len() {
            return Err(InvalidAction::OutOfBounds);
          }
        }
        _ => return Err(InvalidAction::WrongState),
      },
      PlayerMessage::Roll => match &self.state {
        State::FirstRoll => {
          let roll = self.roll_dice();
          messages.push(GameMessage::RollResult(roll));
        }
        State::Reroll => {
          let roll = self.roll_dice_keeping();
          messages.push(GameMessage::RollResult(roll));
        }
        _ => return Err(InvalidAction::WrongState),
      },
      PlayerMessage::Place(num) => match &self.state {
        State::Reroll | State::Place => {
          if *num >= self.ruleset.scorings.len() {
            return Err(InvalidAction::OutOfBounds);
          }

          let next_player = self.player_in_turn + 1;
          let next_round = self.round + 1;
          if next_player < self.players.len() {
            messages.push(GameMessage::PlayerTurn(next_player));
          } else if next_round <= self.ruleset.rounds() {
            messages.push(GameMessage::PlayerTurn(0))
          } else {
            messages.push(GameMessage::GameFinished)
          }
        }
        _ => return Err(InvalidAction::WrongState),
      },
    }

    Ok(messages)
  }

  /// Given a GameMessage returned by `prepare`, commits it to the game state.
  /// Called at the followers to process game state updates from the leader.
  pub fn commit(&mut self, msg: &GameMessage) -> Result<(), InvalidAction> {
    let num_rows = self.ruleset.scorings.len();
    let num_dice = self.ruleset.dice.len();

    match msg {
      GameMessage::PlayerMessage(_from_player, player_msg) => match player_msg {
        PlayerMessage::JoinGame(name) => {
          let name = name.clone();
          let score_sheet = vec![None; num_rows];
          self.players.push(Player { name, score_sheet });
        }
        PlayerMessage::StartGame => {
          // all logic handled in GameMessage::PlayerTurn(0)
        }
        PlayerMessage::ToggleHold(num) => {
          let held = self.keep.get_mut(*num).ok_or(InvalidAction::OutOfBounds)?;
          *held = !*held;
        }
        PlayerMessage::Roll => {
          self.times_rolled = self.times_rolled + 1;
        }
        PlayerMessage::Place(selected_row) => {
          let mut player = self
            .players
            .get_mut(self.player_in_turn)
            .ok_or(InvalidAction::OutOfBounds)?;

          player.score_sheet = update_score_sheet(
            &player.score_sheet,
            &self.ruleset.scorings,
            *selected_row,
            &self.roll,
          )?;
        }
      },
      GameMessage::PlayerTurn(player) => {
        self.times_rolled = 0;
        self.keep = vec![false; num_dice];
        self.roll = vec![1; num_dice];
        self.player_in_turn = *player;
        self.state = State::FirstRoll;
        if self.player_in_turn == 0 {
          self.round += 1;
        }
      }
      GameMessage::RollResult(roll) => {
        self.roll = roll.clone();

        if self.times_rolled < self.ruleset.rolls {
          self.state = State::Reroll;
        } else {
          self.state = State::Place;
        }
      }
      GameMessage::GameFinished => {
        self.state = State::End;
      }
    }

    self.message_history.push(msg.clone());

    Ok(())
  }

  fn roll_dice(&self) -> Vec<u64> {
    roll_dice(&self.ruleset.dice)
  }

  fn roll_dice_keeping(&self) -> Vec<u64> {
    roll_dice_keeping(&self.ruleset.dice, &self.roll, &self.keep)
  }

  pub fn scoreboard(&self) -> Vec<(u64, &str)> {
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
  use crate::rules::mini_rules;

  /// Runs a game and compares its messages to an expected transcript.
  /// All PlayerMessages are sent to the Game, RollResults overwrite the actual roll result, and other messages are checked to be equal.
  fn run_game(game: &mut Game, expected_messages: Vec<GameMessage>) {
    let mut game_messages = VecDeque::new();

    for expected_message in expected_messages {
      println!("{:?}: {:?}", game.state, expected_message);

      match expected_message {
        GameMessage::PlayerMessage(from_player, ref player_msg) => {
          // Player message: Prepare the transaction, commit it and record the messages
          // to check that later messages will match them.
          let mut response_messages: VecDeque<GameMessage> = game
            .prepare(from_player, &player_msg.clone())
            .unwrap()
            .into();

          let actual_player_msg = response_messages.pop_front().unwrap();
          assert_eq!(actual_player_msg, expected_message);
          game.commit(&expected_message).unwrap();
          game_messages.append(&mut response_messages);
        }
        GameMessage::RollResult(..) => {
          // Roll result: Fake the roll result for deterministic testing.
          let actual_roll_message = game_messages.pop_front().unwrap();
          assert!(matches!(actual_roll_message, GameMessage::RollResult(..)));
          game.commit(&expected_message).unwrap();
        }
        _ => {
          // Any other message: Check that the message matches the recorded one.
          let actual_message = game_messages.pop_front().unwrap();
          assert_eq!(actual_message, expected_message);
          game.commit(&expected_message).unwrap();
        }
      }
    }

    assert!(
      game_messages.is_empty(),
      "leftover actual messages without expected counterpart: {:?}",
      game_messages
    );
  }

  #[test]
  fn test_full_game() {
    let mut game = Game::new(mini_rules());
    let (sixes, _bonus, full_house, small_straight) = (0, 1, 2, 3);

    run_game(
      &mut game,
      vec![
        GameMessage::PlayerMessage(0, PlayerMessage::JoinGame("Henry".into())),
        GameMessage::PlayerMessage(1, PlayerMessage::JoinGame("Bobby".into())),
        GameMessage::PlayerMessage(0, PlayerMessage::StartGame),
        // Round 1: Fight!
        GameMessage::PlayerTurn(0),
        GameMessage::PlayerMessage(0, PlayerMessage::Roll),
        GameMessage::RollResult(vec![5, 3, 4, 5, 2]), // bah! puny small straight! let's keep the 5 5 and reroll the 4 3 2
        GameMessage::PlayerMessage(0, PlayerMessage::ToggleHold(0)),
        GameMessage::PlayerMessage(0, PlayerMessage::ToggleHold(3)),
        GameMessage::PlayerMessage(0, PlayerMessage::Roll),
        GameMessage::RollResult(vec![5, 5, 1, 5, 1]),
        GameMessage::PlayerMessage(0, PlayerMessage::Place(full_house)),
        GameMessage::PlayerTurn(1),
        GameMessage::PlayerMessage(1, PlayerMessage::Roll),
        GameMessage::RollResult(vec![2, 3, 2, 4, 1]),
        GameMessage::PlayerMessage(1, PlayerMessage::Place(full_house)),
        // Round 2
        GameMessage::PlayerTurn(0),
        GameMessage::PlayerMessage(0, PlayerMessage::Roll),
        GameMessage::RollResult(vec![6, 6, 6, 6, 5]),
        GameMessage::PlayerMessage(0, PlayerMessage::Place(sixes)),
        GameMessage::PlayerTurn(1),
        GameMessage::PlayerMessage(1, PlayerMessage::Roll),
        GameMessage::RollResult(vec![6, 4, 5, 1, 1]),
        GameMessage::PlayerMessage(1, PlayerMessage::ToggleHold(0)),
        GameMessage::PlayerMessage(1, PlayerMessage::ToggleHold(1)),
        GameMessage::PlayerMessage(1, PlayerMessage::ToggleHold(2)),
        GameMessage::PlayerMessage(1, PlayerMessage::Roll),
        GameMessage::RollResult(vec![6, 4, 5, 1, 2]),
        GameMessage::PlayerMessage(1, PlayerMessage::Place(small_straight)),
        // Round 3
        GameMessage::PlayerTurn(0),
        GameMessage::PlayerMessage(0, PlayerMessage::Roll),
        GameMessage::RollResult(vec![2, 2, 1, 1, 1]),
        GameMessage::PlayerMessage(0, PlayerMessage::ToggleHold(1)),
        GameMessage::PlayerMessage(0, PlayerMessage::ToggleHold(2)),
        GameMessage::PlayerMessage(0, PlayerMessage::Roll),
        GameMessage::RollResult(vec![5, 2, 1, 5, 6]),
        GameMessage::PlayerMessage(0, PlayerMessage::Roll),
        GameMessage::RollResult(vec![3, 2, 1, 6, 6]), // drats! no small straight :(
        GameMessage::PlayerMessage(0, PlayerMessage::Place(small_straight)),
        GameMessage::PlayerTurn(1),
        GameMessage::PlayerMessage(1, PlayerMessage::Roll),
        GameMessage::RollResult(vec![1, 5, 1, 6, 2]),
        GameMessage::PlayerMessage(1, PlayerMessage::ToggleHold(3)),
        GameMessage::PlayerMessage(1, PlayerMessage::Roll),
        GameMessage::RollResult(vec![2, 4, 6, 6, 2]),
        GameMessage::PlayerMessage(1, PlayerMessage::ToggleHold(2)),
        GameMessage::PlayerMessage(1, PlayerMessage::Roll),
        GameMessage::RollResult(vec![1, 6, 6, 6, 1]), // drats! no 4x6 needed for bonus :(
        GameMessage::PlayerMessage(1, PlayerMessage::Place(sixes)),
        GameMessage::GameFinished,
      ],
    );

    assert_eq!(
      game.players[0].score_sheet,
      [Some(24), Some(50), Some(25), Some(0)],
      "player 0 score sheet"
    );
    assert_eq!(
      game.players[1].score_sheet,
      [Some(18), Some(0), Some(0), Some(0)],
      "player 1 score sheet"
    );
    assert_eq!(
      game.scoreboard(),
      [(24 + 50 + 25 + 0, "Henry"), (18 + 0 + 0 + 0, "Bobby")]
    );

    let mut game2 = Game::new(mini_rules());
    for message in game.message_history.iter() {
      game2.commit(message).unwrap();
    }
    assert_eq!(
      game, game2,
      "a game produced by replaying another's message history should be equal to the original one"
    );
  }
}
