enum Scoring {
  Numbers { num: u64 },
  Bonus { min_points: u64, value: u64 },
  SetOf { num: u64 },
  Straight { min_length: u64, value: u64 },
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

      for i in rest {
        if *i == y - 1 {
          y = *i;
          required -= 1;
        }
      }

      required <= 0
    }
    [] => false,
  }
}

impl Scoring {
  fn name(&self) -> String {
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
      FullHouse { .. } => "Full House".into(),
      Yahtzee { .. } => "Yahtzee".into(),
      Chance { .. } => "Chance".into(),
    }
  }

  fn score(&self, roll: &[u64]) -> u64 {
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

      FullHouse { value } => match *roll {
        [x, y, z, w, h] if x == y && z == w && w == h => value,
        [x, y, z, w, h] if x == y && y == z && w == h => value,
        _ => 0,
      },

      Yahtzee { value } => match roll {
        [x, rest @ ..] => {
          if rest.iter().all(|&y| *x == y) {
            value
          } else {
            0
          }
        }
        [] => 0,
      },

      Chance => roll.iter().sum(),
    }
  }
}

struct Ruleset {
  dice: Vec<u64>,
  scorings: Vec<Scoring>,
}

fn basic_rules() -> Ruleset {
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
      // NOTE: Source has a different meaning for small and large straight.
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
  }
}

#[derive(PartialEq, Debug)]
enum UpdateErr {
  OutOfBounds,
  AlreadyOccupied,
  NotSelectable,
}

use UpdateErr::*;

fn update_score_sheet(
  score_sheet: &[Option<u64>],
  scorings: &[Scoring],
  selected_row: usize,
  roll: &[u64],
) -> Result<Vec<Option<u64>>, UpdateErr> {
  let scoring = scorings.get(selected_row);
  // if let None = scoring { return Err(OutOfBounds) }
  // let current_occupant = score_sheet.get(selected_row);

  // if let

  Err(OutOfBounds)
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
    let scorings = vec![
      Numbers { num: 1 },
      Numbers { num: 2 },
      Bonus {
        min_points: 8,
        value: 10,
      },
      Yahtzee { value: 50 },
    ];

    let before: Vec<Option<u64>> = vec![Some(3), None, None, None];
    let expected: Vec<Option<u64>> = vec![Some(3), Some(6), Some(8), None];
    let actual = update_score_sheet(&before, &scorings, 1, &[2, 2, 2, 3, 4]).unwrap();
    assert_eq!(actual, expected);
  }
}
