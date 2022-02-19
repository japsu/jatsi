use itertools::izip;
use rand::Rng;

pub fn roll_dice(dice: &[u64]) -> Vec<u64> {
  let mut rng = rand::thread_rng();
  dice
    .iter()
    .map(|&sides| rng.gen_range(1..=sides))
    .collect::<Vec<u64>>()
}

pub fn roll_dice_keeping(dice: &[u64], old_roll: &[u64], keep: &[bool]) -> Vec<u64> {
  let mut rng = rand::thread_rng();
  izip!(dice, old_roll, keep)
    .map(|(&sides, &old_value, &kept)| {
      if kept {
        old_value
      } else {
        rng.gen_range(1..=sides)
      }
    })
    .collect::<Vec<u64>>()
}
