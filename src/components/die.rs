use itertools::izip;
use yew::{events::MouseEvent, function_component, html, Callback, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct DieProps {
  pub value: u64,
  pub keep: bool,
  pub onclick: Callback<MouseEvent>,
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
const DOT_RADIUS: &str = "200";
const HELD_COLOR: &str = "#aaa";
const UNHELD_COLOR: &str = "#ddd";

// A six-sided die (D6) with dots.
#[function_component(Die)]
pub fn die(props: &DieProps) -> Html {
  let active_dots = &DOTS_FOR_VALUE[(props.value - 1) as usize];
  let fill = if props.keep { HELD_COLOR } else { UNHELD_COLOR };

  html! {
    <svg class="die" viewBox="-1000 -1000 2000 2000" onclick={&props.onclick}>
      <rect x="-1000" y="-1000" width="2000" height="2000" rx={DOT_RADIUS} fill={fill} />
      {
        izip!(&DOTS, active_dots)
          .map(|((x, y), &active)| {
            let cx = x * OFFSET;
            let cy = y * OFFSET;
            if active {
              html! { <circle cx={cx.to_string()} cy={cy.to_string()} r={DOT_RADIUS} fill="#333" /> }
            } else {
              html! {}
            }
          })
          .collect::<Html>()
      }
    </svg>
  }
}
