use crate::errors::InvalidAction;

#[derive(Debug, PartialEq, Clone)]
pub enum Scoring {
  Numbers { num: u64 },
  Bonus { min_points: u64, value: u64 },
  SetOf { num: u64 },
  Straight { min_length: u64, value: u64 },
  TwoPairs {},
  FullHouse { value: u64 },
  Yahtzee { value: u64 },
  Chance,
}

use Scoring::*;

fn is_straight_of_at_least(min_length: u64, roll: &[u64]) -> bool {
  match roll {
    [x, rest @ ..] => {
      let mut y = *x;

      // if we have a straight longer than required, this will go negative, hence i64
      let mut required = min_length as i64 - 1;

      for &i in rest {
        if i == y - 1 {
          y = i;
          required -= 1;
        }
      }

      required <= 0
    }
    [] => false,
  }
}

impl Scoring {
  pub fn name(&self) -> String {
    match *self {
      Numbers { num } => match num {
        1 => "Ones".into(),
        2 => "Twos".into(),
        3 => "Threes".into(),
        4 => "Fours".into(),
        5 => "Fives".into(),
        6 => "Sixes".into(),
        n => format!("{}'s", n),
      },

      Bonus { .. } => "Bonus".into(),

      SetOf { num } => match num {
        2 => "Pair".into(),
        n => format!("Set of {}", n),
      },

      Straight { min_length, .. } => match min_length {
        4 => "Small Straight".into(),
        5 => "Large Straight".into(),
        n => format!("Straight of {}", n),
      },

      TwoPairs { .. } => "Two Pairs".into(),
      FullHouse { .. } => "Full House".into(),
      Yahtzee { .. } => "Yahtzee".into(),
      Chance { .. } => "Chance".into(),
    }
  }

  pub fn score(&self, roll: &[u64]) -> u64 {
    match *self {
      Numbers { num } => roll.iter().filter(|&&x| x == num).sum(),

      Bonus { .. } => 0, // TODO

      SetOf { num } => match roll {
        [x, rest @ ..] => {
          let instances = (rest.iter().filter(|&&y| *x == y).count() + 1) as u64;

          if instances >= num {
            *x * num
          } else {
            self.score(rest)
          }
        }
        [] => 0,
      },

      Straight { min_length, value } => {
        if is_straight_of_at_least(min_length, roll) {
          value
        } else {
          0
        }
      }

      // Not used by *ee rules but used by *y rules
      TwoPairs {} => {
        let mut roll = roll.to_vec();
        roll.sort();
        roll.reverse();

        match *roll {
          // remember: they're ordered
          [x, y, z, w, _] if x == y && z == w => x + y + z + w,
          [x, y, _, w, h] if x == y && w == h => x + y + w + h,
          [_, y, z, w, h] if y == z && w == h => y + z + w + h,
          _ => 0,
        }
      }

      FullHouse { value } => {
        let mut roll = roll.to_vec();
        roll.sort();
        roll.reverse();
        match *roll {
          [x, y, z, w, h] if x == y && z == w && w == h => value,
          [x, y, z, w, h] if x == y && y == z && w == h => value,
          _ => 0,
        }
      }

      Yahtzee { value } => {
        if roll.iter().all(|&x| x == roll[0]) {
          value
        } else {
          0
        }
      }

      Chance => roll.iter().sum(),
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum JokerRule {
  Forced,
  FreeChoice,
  Original,
  NoJoker,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ruleset {
  pub dice: Vec<u64>,
  pub scorings: Vec<Scoring>,
  pub joker_rule: JokerRule, // not implemented yet
  pub rolls: u64,            // 3
}

/// Mini ruleset. Mainly useful for testing.
pub fn mini_rules() -> Ruleset {
  Ruleset {
    dice: vec![6; 5],
    scorings: vec![
      Numbers { num: 6 },
      Bonus {
        min_points: 24,
        value: 50,
      },
      FullHouse { value: 25 },
      Straight {
        min_length: 4,
        value: 30,
      },
    ],
    joker_rule: JokerRule::Forced,
    rolls: 3,
  }
}

pub fn ee_rules() -> Ruleset {
  Ruleset {
    dice: vec![6; 5],
    scorings: vec![
      Numbers { num: 1 },
      Numbers { num: 2 },
      Numbers { num: 3 },
      Numbers { num: 4 },
      Numbers { num: 5 },
      Numbers { num: 6 },
      Bonus {
        min_points: 63,
        value: 50,
      },
      SetOf { num: 3 },
      SetOf { num: 4 },
      FullHouse { value: 25 },
      Straight {
        min_length: 4,
        value: 30,
      },
      Straight {
        min_length: 5,
        value: 40,
      },
      Chance {},
      Yahtzee { value: 50 },
    ],
    joker_rule: JokerRule::Forced,
    rolls: 3,
  }
}

fn roleplayers_rules() -> Ruleset {
  Ruleset {
    dice: vec![4, 6, 8, 10, 10],
    scorings: vec![
      Numbers { num: 1 },
      Numbers { num: 2 },
      Numbers { num: 3 },
      Numbers { num: 4 },
      Numbers { num: 5 },
      Numbers { num: 6 },
      Numbers { num: 7 },
      Numbers { num: 8 },
      Numbers { num: 9 },
      Numbers { num: 10 },
      Bonus {
        min_points: 105,
        value: 50,
      },
      SetOf { num: 3 },
      SetOf { num: 4 },
      FullHouse { value: 25 },
      // NOTE Source has a different meaning for small and large straight.
      // TODO Small straight to mean 1-5 or 2-6 and large 3-7 or 4-8
      // TODO 5-9 and 6-10 are possible, albeit improbable. Allow them too?
      Straight {
        min_length: 4,
        value: 30,
      },
      Straight {
        min_length: 5,
        value: 40,
      },
      Chance {},
      Yahtzee { value: 50 },
    ],
    joker_rule: JokerRule::Forced,
    rolls: 3,
  }
}

impl Ruleset {
  pub fn rounds(&self) -> usize {
    self
      .scorings
      .iter()
      .filter(|item| if let Bonus { .. } = item { false } else { true })
      .count()
  }
}

pub fn update_score_sheet(
  score_sheet: &[Option<u64>],
  scorings: &[Scoring],
  selected_index: usize,
  roll: &[u64],
) -> Result<Vec<Option<u64>>, InvalidAction> {
  let scoring = scorings
    .get(selected_index)
    .ok_or(InvalidAction::OutOfBounds)?;
  let &current_row = score_sheet
    .get(selected_index)
    .ok_or(InvalidAction::OutOfBounds)?;

  let mut roll_points = scoring.score(roll);

  // The Bonus row cannot be selected.
  if let Bonus { .. } = scoring {
    return Err(InvalidAction::NotSelectable);
  }

  // Usually you cannot choose the same row twice.
  // The Yahtzee row is an exception: if you get it multiple times, it accumulates.
  if let Some(existing_points) = current_row {
    if let Yahtzee { .. } = scoring {
      if roll_points > 0 && existing_points > 0 {
        roll_points += existing_points
      } else {
        return Err(InvalidAction::AlreadyOccupied);
      }
    } else {
      return Err(InvalidAction::AlreadyOccupied);
    }
  }

  let mut new_score_sheet = score_sheet.to_vec();
  new_score_sheet[selected_index] = Some(roll_points);

  // The bonus is scored when either all rows above it are scored, or when the threshold is exceeded.
  // Not all scoring systems have a bonus row.
  let maybe_bonus_row = scorings.iter().enumerate().find(|(_, scoring)| {
    if let Bonus { .. } = scoring {
      true
    } else {
      false
    }
  });
  if let Some((bonus_index, Bonus { min_points, value })) = maybe_bonus_row {
    // Our scoring rules have a bonus row
    let bonus_affecting_rows = &new_score_sheet[..bonus_index];
    let all_bonus_affecting_rows_filled = bonus_affecting_rows.iter().all(|&row| row.is_some());
    let bonus_threshold_crossed = bonus_affecting_rows
      .iter()
      .map(|&row| if let Some(pts) = row { pts } else { 0 })
      .sum::<u64>()
      >= *min_points;

    if all_bonus_affecting_rows_filled || bonus_threshold_crossed {
      // Bonus will be scored now
      new_score_sheet[bonus_index] = Some(if bonus_threshold_crossed { *value } else { 0 })
    }
  }

  Ok(new_score_sheet)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_numbers() {
    let ones = Numbers { num: 1 };

    assert_eq!(ones.name(), "Ones");
    assert_eq!(ones.score(&[1, 1, 1, 1, 1]), 5, "All ones");
    assert_eq!(ones.score(&[1, 1, 1, 2, 3]), 3, "Some ones");
  }

  #[test]
  fn test_pair() {
    let pair = SetOf { num: 2 };

    assert_eq!(pair.name(), "Pair");

    assert_eq!(
      pair.score(&[5, 4, 3, 2, 1]),
      0,
      "A roll with no pair gives zero points"
    );
    assert_eq!(
      pair.score(&[1, 1, 1, 1, 1]),
      2,
      "A roll with a pair gives points equal to the sum of the values of the pair"
    );
    assert_eq!(
      pair.score(&[4, 3, 2, 1, 1]),
      2,
      "The set need not be at the start of the roll",
    );
    assert_eq!(
      pair.score(&[3, 2, 2, 1, 1]),
      4,
      "If multiple pairs are present, the value of the highest one is returned"
    )
  }

  #[test]
  fn test_set_of_three() {
    let set_of_three = SetOf { num: 3 };

    assert_eq!(set_of_three.name(), "Set of 3");
    assert_eq!(set_of_three.score(&[5, 4, 4, 4, 3]), 12);
  }

  #[test]
  fn test_small_straight() {
    let small_straight = Straight {
      min_length: 4,
      value: 30,
    };

    assert_eq!(small_straight.name(), "Small Straight");
    assert_eq!(
      small_straight.score(&[6, 5, 3, 2, 1]),
      0,
      "Not a small straight"
    );
    assert_eq!(
      small_straight.score(&[6, 5, 4, 3, 1]),
      30,
      "Upper small straight at beginning",
    );
    assert_eq!(
      small_straight.score(&[6, 5, 4, 3, 3]),
      30,
      "Upper small straight at beginning with dupe at the end",
    );
    assert_eq!(
      small_straight.score(&[6, 5, 4, 4, 3]),
      30,
      "Upper small straight with dupe in the middle",
    );
    assert_eq!(
      small_straight.score(&[6, 5, 4, 3, 2]),
      30,
      "A large straight is also a small straight"
    )
  }

  #[test]
  fn test_large_straight() {
    let large_straight = Straight {
      min_length: 5,
      value: 40,
    };

    assert_eq!(
      large_straight.score(&[6, 5, 4, 3, 2]),
      40,
      "Upper large straight"
    );

    assert_eq!(
      large_straight.score(&[5, 4, 3, 2, 1]),
      40,
      "Lower large straight"
    );

    assert_eq!(
      large_straight.score(&[5, 4, 3, 1, 1]),
      0,
      "Not a large straight"
    );
  }

  #[test]
  fn test_two_pairs() {
    let two_pairs = TwoPairs {};

    assert_eq!(two_pairs.score(&[6, 5, 5, 4, 1]), 0, "Only one pair");
    assert_eq!(two_pairs.score(&[5, 4, 3, 2, 1]), 0, "Not two pairs");

    assert_eq!(
      two_pairs.score(&[5, 3, 3, 1, 1]),
      8,
      "Two pairs with orphan at the start"
    );
    assert_eq!(
      two_pairs.score(&[4, 4, 3, 2, 2]),
      12,
      "Two pairs with orphan in the middle"
    );
    assert_eq!(
      two_pairs.score(&[6, 6, 2, 2, 1]),
      16,
      "Two pairs with orphan at the end"
    );

    assert_eq!(
      two_pairs.score(&[3, 3, 3, 2, 2]),
      10,
      "Full house with triplet at the start also counts as two pairs"
    );
    assert_eq!(
      two_pairs.score(&[6, 6, 1, 1, 1]),
      14,
      "Full house with triplet at the end also counts as two pairs"
    );
  }

  #[test]
  fn test_full_house() {
    let full_house = FullHouse { value: 25 };

    assert_eq!(full_house.score(&[4, 3, 2, 2, 1]), 0, "Not a full house");
    assert_eq!(
      full_house.score(&[3, 3, 3, 2, 2]),
      25,
      "Full house with triplet at the start"
    );
    assert_eq!(
      full_house.score(&[6, 6, 1, 1, 1]),
      25,
      "Full house with triplet at the end"
    );
  }

  #[test]
  fn test_yahtzee() {
    let yahtzee = Yahtzee { value: 50 };

    assert_eq!(yahtzee.score(&[5, 5, 5, 5, 5]), 50, "Yahtzee!!!");
    assert_eq!(yahtzee.score(&[5, 5, 3, 5, 5]), 0, "Not a yahtzee");
    assert_eq!(yahtzee.score(&[5, 5, 5, 5, 3]), 0, "Also not a yahtzee");
    assert_eq!(yahtzee.score(&[3, 5, 5, 5, 5]), 0, "Nor is this yahtzee");
  }

  #[test]
  fn test_chance() {
    let chance = Chance {};
    assert_eq!(chance.score(&[6, 5, 4, 4, 2]), 21);
  }

  #[test]
  fn test_bonus() {
    let value = 10;
    let scorings = vec![
      Numbers { num: 1 },
      Numbers { num: 2 },
      Bonus {
        min_points: 8,
        value,
      },
      Yahtzee { value: 50 },
    ];

    let before: Vec<Option<u64>> = vec![Some(3), None, None, None];
    let expected: Vec<Option<u64>> = vec![Some(3), Some(6), Some(value), None];
    let actual = update_score_sheet(&before, &scorings, 1, &[2, 2, 2, 3, 4]).unwrap();
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_full_game() {
    let scorings = vec![
      Numbers { num: 1 }, // 0
      Numbers { num: 2 }, // 1
      Bonus {
        // 2
        min_points: 10,
        value: 50,
      },
      SetOf { num: 3 },        // 3
      FullHouse { value: 25 }, // 4
      Straight {
        // 5
        min_length: 4,
        value: 30,
      },
      Straight {
        // 6
        min_length: 5,
        value: 40,
      },
      Chance {},             // 7
      Yahtzee { value: 50 }, // 8
    ];

    let mut score_sheet: Vec<Option<u64>> = vec![None; scorings.len()];

    // 8 - Yahtzee
    score_sheet = update_score_sheet(&score_sheet, &scorings, 8, &[6, 6, 6, 6, 6]).unwrap();
    assert_eq!(
      score_sheet,
      [None, None, None, None, None, None, None, None, Some(50)]
    );

    // 0 - Ones
    score_sheet = update_score_sheet(&score_sheet, &scorings, 0, &[6, 6, 1, 1, 1]).unwrap();
    assert_eq!(
      score_sheet,
      [Some(3), None, None, None, None, None, None, None, Some(50)]
    );

    // 3 - Set of 3
    score_sheet = update_score_sheet(&score_sheet, &scorings, 3, &[6, 6, 5, 5, 5]).unwrap();
    assert_eq!(
      score_sheet,
      [
        Some(3),
        None,
        None,
        Some(15),
        None,
        None,
        None,
        None,
        Some(50)
      ]
    );

    // 5 - Straight
    score_sheet = update_score_sheet(&score_sheet, &scorings, 5, &[5, 5, 4, 3, 2]).unwrap();
    assert_eq!(
      score_sheet,
      [
        Some(3),
        None,
        None,
        Some(15),
        None,
        Some(30),
        None,
        None,
        Some(50)
      ]
    );

    // 8 - OMG! YAHTZEE AGAIN!
    score_sheet = update_score_sheet(&score_sheet, &scorings, 8, &[1, 1, 1, 1, 1]).unwrap();
    assert_eq!(
      score_sheet,
      [
        Some(3),
        None,
        None,
        Some(15),
        None,
        Some(30),
        None,
        None,
        Some(100)
      ]
    );

    // 1 - Twos
    score_sheet = update_score_sheet(&score_sheet, &scorings, 1, &[1, 2, 2, 2, 2]).unwrap();

    // Bonus should be scored at this point!
    assert_eq!(
      score_sheet,
      [
        Some(3),
        Some(8),
        Some(50),
        Some(15),
        None,
        Some(30),
        None,
        None,
        Some(100)
      ]
    );

    // TODO finish game :)
  }
}
