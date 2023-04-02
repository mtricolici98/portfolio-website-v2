mod app;

pub mod components;
pub mod routed;
pub mod data;
pub mod hooks;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
