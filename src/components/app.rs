use yew::{function_component, html};

use super::die::Die;
use super::regular_polygon::RegularPolygon;

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <main>
      <svg viewBox="-1000 -1000 2000 2000" width={300} height={300}>
        /*<RegularPolygon p={3} r={200} />
        <text x={0} y={0} fill="#333" font-size="72pt" text-anchor="middle" dominant-baseline="middle">{6}</text>*/
        <Die value=6 />
      </svg>
    </main>
  }
}
