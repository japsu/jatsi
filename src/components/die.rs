use dioxus::{events::MouseEvent, prelude::*};
use itertools::izip;

#[derive(Props)]
pub struct DieProps<'a> {
  pub value: u64,
  pub keep: bool,
  pub onclick: EventHandler<'a, MouseEvent>,
  // pub onclick: Callback<MouseEvent>,
}

const DOTS: [(i64, i64); 7] = [(-1, -1), (-1, -0), (-1, 1), (1, -1), (1, 0), (1, 1), (0, 0)];
const DOTS_FOR_VALUE: [[bool; 7]; 6] = [
  [false, false, false, false, false, false, true],
  [false, false, true, true, false, false, false],
  [false, false, true, true, false, false, true],
  [true, false, true, true, false, true, false],
  [true, false, true, true, false, true, true],
  [true, true, true, true, true, true, false],
];
const OFFSET: i64 = 600;
const DOT_RADIUS: u64 = 200;
const HELD_COLOR: &str = "#aaa";
const UNHELD_COLOR: &str = "#ddd";

// A six-sided die (D6) with dots.
#[allow(non_snake_case)]
pub fn Die<'a>(cx: Scope<'a, DieProps<'a>>) -> Element {
  let DieProps {
    value,
    keep,
    onclick,
  } = cx.props;

  let active_dots = &DOTS_FOR_VALUE[(value - 1) as usize];
  let fill = if *keep { HELD_COLOR } else { UNHELD_COLOR };
  let dots = izip!(&DOTS, active_dots)
    .filter(|(_, &active)| active)
    .map(|((x, y), _)| {
      let cx = x * OFFSET;
      let cy = y * OFFSET;
      rsx!(circle {
        cx: "{cx}",
        cy: "{cy}",
        r: "{DOT_RADIUS}",
        fill: "#333"
      })
    });

  let sr_text = format!(
    "Die, value {}, {}",
    value,
    if *keep {
      "held. Click to release."
    } else {
      "not held. Click to hold."
    }
  );

  rsx!(cx,
    button {
      class: "die",
      onclick: |e| onclick.call(e),
      prevent_default: "onclick",
      title: "{sr_text}",

      svg {
        view_box: "-1000 -1000 2000 2000",

        rect {
          x: "-1000",
          y: "-1000",
          width: "2000",
          height: "2000",
          rx: "{DOT_RADIUS}",
          fill: "{fill}",
        }

        dots
      }
    }
  )
}
