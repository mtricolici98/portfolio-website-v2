
use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlMediaElement;
use yew::prelude::*;
use yew_hooks::{use_toggle, use_counter, use_interval};

#[derive(Properties, PartialEq, Clone)]
pub struct VideoPlayerProps { 
    pub name: String,
    pub url: String
}



#[function_component]
pub fn VideoPlayer(props: &VideoPlayerProps) -> Html {
    let paused = use_toggle(false, true);
    let _paused = paused.clone();
    let on_play_pause_click = {
        Callback::from(move |_: MouseEvent| {
            let paused = _paused.clone();
            paused.toggle();
            let window = web_sys::window().expect("global window does not exists");    
            let document = window.document().expect("expecting a document on window");
            let player = document.get_element_by_id("vid-player").unwrap()
            .dyn_into::<web_sys::HtmlMediaElement>().unwrap();
            if *paused {
                player.play().unwrap();
            } else {
                player.pause().unwrap();
            }
        })
    };

    let progress = use_counter(0);

    let progress_style = format!{"width: {}%; transition: width 1s ease-in-out;", *progress.clone()};
    {
        use_interval(move || {
            let window = web_sys::window().expect("global window does not exists");    
            let document = window.document().expect("expecting a document on window");
            let player = document.get_element_by_id("vid-player");
            match player {
                Some(some_player) => {
                    let some_player = some_player.dyn_into::<web_sys::HtmlMediaElement>().unwrap();
                    let new_progress = some_player.current_time() / some_player.duration();
                    progress.set((new_progress * 100.0) as u32 as i32);
                    log!{"Set progress to", *progress};
                }, 
                _ => {}
            }
        }, 1000);
    }
    let element = html! {
        <div>
            <div class="flex w-full bg-white shadow-md rounded-lg overflow-hidden mx-auto">
                <div class="flex flex-col m-5 ">
                    <div class="relative">
                        <video class="w-screen" autoplay={true} muted={true} loop={true} id="vid-player">
                            <source src={props.url.clone()}/>
                        </video>
                        <div class="relative h-1 bg-gray-200">
                            <div class="absolute h-full bg-accent flex items-center justify-end" style={progress_style}>
                            </div>
                        </div>
                        <div class="flex justify-center text-xs font-medium text-gray-500 py-1">
                            <div class="flex space-x-3 pt-5">
                            <button onclick={on_play_pause_click}
                            class="rounded-full w-8 h-8 flex items-center justify-center">
                                    if *paused {
                                        <svg xmlns="http://www.w3.org/2000/svg" id="play-icon" version="1.1" height="50" width="50" viewBox="0 0 1200 1200"><path fill="#6c3d36" d="M 600,1200 C 268.65,1200 0,931.35 0,600 0,268.65 268.65,0 600,0 c 331.35,0 600,268.65 600,600 0,331.35 -268.65,600 -600,600 z M 450,300.45 450,899.55 900,600 450,300.45 z" id="path16995" /></svg>
                                    } else {
                                        <svg xmlns="http://www.w3.org/2000/svg" id="pause-icon" version="1.1" height="50" width="50" viewBox="0 0 1200 1200"><path fill="#6c3d36" id="path15778" d="M 600,0 C 268.62914,0 0,268.62914 0,600 c 0,331.37086 268.62914,600 600,600 331.37086,0 600,-268.62914 600,-600 C 1200,268.62914 931.37086,0 600,0 z m -269.16515,289.38 181.71397,0 0,621.24 -181.71397,0 0,-621.24 z m 356.61633,0 181.71399,0 0,621.24 -181.71399,0 0,-621.24 z" /></svg>
                                    }
                            </button>
                        </div>
                        </div>
                    </div>
                <div>
            </div>
        </div>
    </div>
</div>
    };
    element
}