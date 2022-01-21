trait Scoring {
  fn name(&self) -> String;
  fn score(&self, roll: &[u64]) -> u64;
}

struct SetOf {
  num: u64,
}

impl Scoring for SetOf {
  fn name(&self) -> String {
    match self.num {
      2 => "Pair".into(),
      n => format!("Set of {}", n),
    }
  }

  fn score(&self, roll: &[u64]) -> u64 {
    match roll {
      [x, rest @ ..] => {
        let mut instances = 1;

        for y in rest {
          if x == y {
            instances += 1;
          }
        }

        if instances >= self.num {
          x * self.num
        } else {
          self.score(rest)
        }
      }
      [] => 0,
    }
  }
}

struct Straight {
  min_length: u64,
}

/// Find longest flush in `roll`. Initially set `acc` to 1 and `best` to 0.
fn longest_flush(roll: &[u64], acc: u64, best: u64) -> u64 {
  use std::cmp::max;

  match roll {
    [x, rest @ ..] => match rest {
      [y, ..] => {
        if *x == *y + 1 {
          longest_flush(rest, acc + 1, best)
        } else {
          longest_flush(rest, 1, max(acc, best))
        }
      }
      [] => max(acc, best),
    },
    [] => max(acc, best),
  }
}

impl Scoring for Straight {
  fn name(&self) -> String {
    match self.min_length {
      4 => "Small Straight".into(),
      5 => "Large Straight".into(),
      n => format!("Straight of {}", n),
    }
  }

  fn score(&self, roll: &[u64]) -> u64 {
    if longest_flush(roll, 1, 0) >= self.min_length {
      (self.min_length - 1) * 10
    } else {
      0
    }
  }
}

struct FullHouse {}

impl Scoring for FullHouse {
  fn name(&self) -> String {
    "Full House".into()
  }

  fn score(&self, roll: &[u64]) -> u64 {
    match roll {
      [x, y, z, w, h] if *x == *y && *z == *w && *w == *h => 25,
      [x, y, z, w, h] if *x == *y && *y == *z && *w == *h => 25,
      _ => 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
  fn test_flush_length() {
    // trivial cases
    assert_eq!(longest_flush(&[5, 4, 3], 1, 0), 3);
    assert_eq!(longest_flush(&[5, 4, 3, 2], 1, 0), 4);

    // flush at start
    assert_eq!(longest_flush(&[6, 5, 4, 3, 1], 1, 0), 4);

    // flush not at the start
    assert_eq!(longest_flush(&[5, 5, 4, 3, 2], 1, 0), 4);
    assert_eq!(longest_flush(&[6, 4, 3, 2, 1], 1, 0), 4);

    // flush of four in middle :)
    assert_eq!(longest_flush(&[9, 7, 6, 5, 4, 2, 1], 1, 0), 4);

    // dupe in the middle :(
    assert_eq!(longest_flush(&[6, 5, 4, 4, 3], 1, 0), 4);
  }

  #[test]
  fn test_small_straight() {
    let small_straight = Straight { min_length: 4 };

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
    )
  }

  #[test]
  fn test_large_straight() {
    let large_straight = Straight { min_length: 5 };

    // assert_eq!(
    //   large_straight
    // )
  }

  #[test]
  fn test_full_house() {
    let full_house = FullHouse {};

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
}
