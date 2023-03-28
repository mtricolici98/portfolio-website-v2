use yew::prelude::*;

use crate::{routed::RouterMain, components::{navigation::top_nav::Navigation, other::misc::Footer}};


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="flex flex-col bg-main h-screen">
            <section class="flex-1 grow-0">
                <Navigation />
            </section>
            <div class="flex-1 grow">
                <RouterMain />
            </div>
            <section class="flex-1 grow-0">
                <Footer />
            </section>
        </main>
    }
}
