use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{other::{misc::{AboutMe, Pricing}, gallery::{GalleryGrid, GalleryGridProps, Gallery}}, home::Home};


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/pricing")]
    Pricing,
    #[at("/gallery")]
    Gallery,
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn route_switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! {
            <AboutMe />
        },
        Route::Gallery => html! {
            <Gallery/>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        _ =>  html! {
            <Redirect<Route> to={Route::NotFound}/>
        }
    }
}

#[function_component]
pub fn RouterMain() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={route_switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}