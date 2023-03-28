use yew::prelude::*;

#[function_component]
pub fn Navigation() -> Html {
    return html! { 
        
<nav class="border-gray-200 px-2 sm:px-4 py-2.5 rounded">
    <div class="container flex flex-wrap items-center justify-center mx-auto">
    <ul class="flex flex-row p-4 mt-4 rounded-lg">
        <li>
            <a href="/gallery" class="block lg:px-12 md:px-6 py-2 text-xl text-accent rounded">{"Gallery"}</a>
        </li>
    </ul>
    <a href="/" class="flex items-center">
      <img src="img/logo/svg/logo-no-background.svg" class="h-16 mr-3 sm:h-16" alt="Logo" />
    </a>
    <ul  class="flex flex-row p-4 mt-4 rounded-lg">
        <li>
            <a href="#" class="block py-2 lg:px-12 md:px-6 text-xl text-accent rounded">{"Contact"}</a>
        </li>
    </ul>
    </div>
</nav>
    }
}