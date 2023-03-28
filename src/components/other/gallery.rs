use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children,
    pub modal_host_id: String,
}

#[function_component]
fn Modal(props: &ModalProps) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id(&props.modal_host_id)
        .expect("Expected to find a #modal_host element");

    create_portal(
        html!{ {for props.children.iter()} },
        modal_host.into(),
    )
}

#[derive(Properties, PartialEq, Clone)]
pub struct GalleryGridElement { 

}

#[function_component]
pub fn GalleryElement(props: &GalleryGridElement) -> Html {
    html! {
        <div
        class="my-8 rounded shadow-lg shadow-gray-200 bg-white duration-300 hover:-translate-y-1"
        x-for="(post, index) in posts">
        <a _href="link" class="cursor-pointer">
            <figure>
                <img src="/img/logo/svg/logo-color.svg" class="rounded-t h-72 w-full object-cover" />

                <figcaption class="p-4">
                    <p
                        class="text-lg mb-4 font-bold leading-relaxed text-gray-800"
                        x-text="post.title">
                        {"Some text"}
                    </p>

                    <small
                        class="leading-5 text-gray-500"
                        x-text="post.description">
                        {"Some descrption..."}
                    </small>
                </figcaption>
            </figure>
        </a>
    </div>
    }
}


// 
//         <template x-for="post in posts">
//             <!-- Card Item -->

//         </template>
//     </div>

#[derive(Properties, PartialEq, Default)]
pub struct GalleryGridProps {
    pub elements: Vec<GalleryGridElement>
}

#[function_component]
pub fn GalleryGrid(props: &GalleryGridProps)  -> Html {
    let elements: Html = props.elements.clone().into_iter()
    .map(|el| {
        html! {<GalleryElement />}
    })
    .collect();
    html! {
        <section>
        <div class="grid grid-flow-row gap-8 text-neutral-600 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                {elements}
            </div>
        </section>
    }
}


#[function_component]
pub fn Gallery() -> Html {
    html! {
        <GalleryGrid ..GalleryGridProps{elements: vec!{GalleryGridElement{}, GalleryGridElement{}}}/>
    }
}