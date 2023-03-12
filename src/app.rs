use yew::prelude::*;

use crate::{routed::RouterMain, components::navigation::top_nav::Navigation};


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Navigation />
            <RouterMain />
        </main>
    }
}
