mod app;

pub mod components;
pub mod routed;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
