use std::collections::{BTreeMap};

use gloo_console::log;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::{use_toggle, UseToggleHandle};

use crate::components::other::video::{VideoPlayer, VideoPlayerProps};
use crate::components::other::modal::{Modal};
use crate::data::gallery_element::{GalleryElementData, GalleryElementsCategory, get_gallery_data_map};


#[derive(Properties, PartialEq, Clone)]
pub struct GalleryGridElement { 
    data: GalleryElementData
}



#[derive(Properties, PartialEq, Clone)]
pub struct GalleryModalProps { 
    data: GalleryElementData,
    toggle: UseToggleHandle<bool>
}




#[function_component]
pub fn GalleryModalComponent(props: &GalleryModalProps ) -> Html {
    let toggle = props.toggle.clone();
    let oncloseclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| {
            log!("click registered on close");
            toggle.set(false);
    })
    };
    let onclick = { 
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target().and_then(|event_target: web_sys::EventTarget| {
                event_target.dyn_into::<web_sys::Element>().ok()
            }) {
                let window = web_sys::window().expect("global window does not exists");    
                let document = window.document().expect("expecting a document on window");
                let inner_modal = document.get_element_by_id("modal-inner");
                match inner_modal {
                    Some(modal_element) => {       
                        let contains = target.contains(Some(modal_element.as_ref()));
                        if contains { 
                            toggle.set(false);
                        }
                    },
                    _ => {}
                } 
            }
        })
    };
    html!{
    <div onclick={onclick} class="fixed top-0 left-0 z-[1055] h-full w-full overflow-y-auto overflow-x-hidden outline-none">
        <div id="modal-inner"
            class="pointer-events-none relative h-[calc(100%-1rem)] w-auto min-[576px]:mx-auto min-[576px]:mt-7 min-[576px]:h-[calc(100%-3.5rem)] min-[576px]:max-w-[500px]">
            <div
            class="pointer-events-auto relative flex max-h-[100%] w-full flex-col overflow-hidden rounded-md border-none bg-white bg-clip-padding text-current shadow-lg outline-none">
            <div
                class="flex flex-shrink-0 items-center justify-between rounded-t-md border-b-2 border-neutral-100 border-opacity-100 p-4">
                <h5
                class="text-xl font-medium leading-normal text-accent">
                {props.data.name.clone()}
                </h5>
                <button
                type="button"
                class="box-content rounded-none border-none hover:no-underline hover:opacity-75 focus:opacity-100 focus:shadow-none focus:outline-none"
                onclick={oncloseclick}
                >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="h-6 w-6">
                    <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M6 18L18 6M6 6l12 12" />
                </svg>
                </button>
            </div>
            <div class="relative overflow-y-auto p-4">
            {
                match props.data.video_url.clone() {
                    Some(url) => {
                        html!{<VideoPlayer ..VideoPlayerProps{url, name: String::from("Demo")}/>}
                    },
                    _ => {
                        html!{}
                    }
                }
            }
            {
                props.data.images_url.clone().into_iter().map(|url| {
                    html!{
                        <img class="h-auto max-w-full shadow-md rounded-lg py-2" src={url}/>
                    }
                }).collect::<Html>()
            }
            </div>
            </div>
        </div>
    </div>
    }
}

#[function_component]
pub fn GalleryElement(props: &GalleryGridElement) -> Html {
    let main_text = props.data.name.clone();
    let desctiption = props.data.desctiption.clone();
    let main_image_url = props.data.main_image_url.clone();

    let show_modal = use_toggle(false, true);

    let onclick = {
        let toggle = show_modal.clone();
        Callback::from(move |_| toggle.toggle())
    };

    html! {
        <div class="my-8 rounded shadow-lg shadow-gray-200 bg-white duration-300 hover:-translate-y-1"
        x-for="(post, index) in posts">
        <a _href="link" class="cursor-pointer" {onclick}>
            <figure>
                <img class="aspect-video transition-all duration-300 rounded-lg
                 cursor-pointer filter saturate-[.25] hover:saturate-100" src={main_image_url}/>
                <figcaption class="p-4">
                    <p class="text-lg mb-4 font-bold leading-relaxed text-accent">
                        {main_text}
                    </p>

                    <small class="leading-5 text-main">
                        {desctiption}
                    </small>
                </figcaption>
            </figure>
        </a>
        if *show_modal {
            <Modal modal_host_id="gallery-section">
                <GalleryModalComponent ..GalleryModalProps{data: props.data.clone(), toggle: show_modal.clone()}/>
            </Modal>
        }
    </div>
    }
}

#[derive(Properties, PartialEq, Default)]
pub struct GalleryGridProps {
    pub elements: Vec<GalleryGridElement>
}

#[function_component]
pub fn GalleryGrid(props: &GalleryGridProps)  -> Html {
    let elements: Html = props.elements.clone().into_iter()
    .map(|el| {
        html! {<GalleryElement ..el/>}
    })
    .collect();
    html! {
        <section id="gallery-section">
        <div class="grid grid-flow-row gap-8 text-neutral-600 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                {elements}
            </div>
        </section>
    }
}


#[derive(Properties, PartialEq, Default)]
pub struct CategorizedGalleryGridProps {
    pub data: BTreeMap<GalleryElementsCategory, Vec<GalleryElementData>>
}


#[function_component]
pub fn CategorisedGalleryGrid(props: &CategorizedGalleryGridProps)  -> Html {
    let inner: Html = props.data.clone().into_iter().map(|pair: (GalleryElementsCategory, Vec<GalleryElementData>)| {
        let (cat_data, elements) = pair;
        html!{ 
            <li class="mb-10 ml-6">            
                <div class="absolute w-3 h-3 bg-gray-200 rounded-full mt-1.5 -left-1.5 border border-white"></div>
                <h3 class="flex items-center mb-1 text-lg font-semibold text-accent">{cat_data.name.clone()} <span class="bg-accent text-white text-sm font-medium mr-2 px-2.5 py-0.5 rounded ml-3">{cat_data.price.clone()}</span></h3>
                <p class="block mb-2 text-sm font-normal leading-none text-main">{cat_data.description.clone()}</p>
                <GalleryGrid ..GalleryGridProps{elements: elements.clone().into_iter().map(|el| {GalleryGridElement{data: el}}).collect()}/>
            </li>
        }
    }).collect();

    html! {
    <ol class="relative border-l border-gray-200 lg:px-48 md:px-24 px-12">                  
        {inner}
    </ol>
    }
}


#[function_component]
pub fn Gallery() -> Html {
    html! {
        <CategorisedGalleryGrid ..CategorizedGalleryGridProps{data: get_gallery_data_map()}/>
    }
}