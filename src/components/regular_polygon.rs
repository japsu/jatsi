use std::f64::consts::PI;
use yew::{function_component, html, Properties};

#[derive(PartialEq, Properties)]
pub struct RegularPolygonProps {
  pub p: u64,
  pub r: u64,
}

/// here be trigonometry
#[function_component(RegularPolygon)]
pub fn regular_polygon(props: &RegularPolygonProps) -> Html {
  let (p, r) = (props.p as f64, props.r as f64);
  let base_angle = 2.0 * PI / p;

  let mut d_attr = "".to_string();
  for i in 0..props.p {
    let angle = i as f64 * base_angle;

    let x = r * angle.cos();
    let y = r * angle.sin();

    d_attr += format!(
      "{}{} {} ",
      if i == 0 { "M" } else { "L" },
      x.floor(),
      y.floor()
    )
    .as_str()
  }

  d_attr += "z";

  html! {
    <path d={d_attr} stroke="#888" fill="#ccc" />
  }
}
