use yew::prelude::*;

#[function_component]
pub fn Home() -> Html { 
    html !{ 
        <section class="bg-main">
        <div class="grid max-w-screen-xl px-4 pt-10 pb-8 mx-auto lg:gap-8 xl:gap-0 lg:grid-cols-12 lg:pt-16">
            <div class="hidden lg:mt-0 lg:col-span-7 lg:flex">
                <img class="" src="img/graphic-main.webp" alt="Main image"/>
            </div>
            <div class="mr-auto place-self-center lg:col-span-5">
                <h1 class="max-w-2xl mb-4 text-4xl font-extrabold leading-none tracking-tight md:text-5xl xl:text-6xl text-accent">{"Turn your"} <br/> {"presentation"} <br/> {"into an event."} </h1>
                <p class="max-w-2xl mb-6 font-light lg:mb-8 md:text-lg lg:text-xl text-main">{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus turpis mauris, tincidunt ut rutrum et, tincidunt non ex. Vestibulum sollicitudin tortor turpis, eu congue nisl hendrerit ac. Morbi sed augue a augue suscipit auctor eget sit amet massa. Pellentesque a dictum nibh, eu varius augue. Curabitur ultricies sit amet massa eu consequat. In hac habitasse platea dictumst. Aenean faucibus non ipsum ac condimentum. Nulla eleifend, purus mollis porta vulputate, lectus neque dapibus ligula, sit amet suscipit sem dolor at nisl. Donec eget egestas est, eleifend consectetur lacus. Nulla ultricies, urna ut fringilla luctus, massa lectus molestie nulla, nec sodales libero ante non dolor."}</p>
                <div class="space-y-4 sm:flex sm:space-y-0 sm:space-x-4">
                    // <a href="https://github.com/themesberg/landwind" class="inline-flex items-center justify-center w-full px-5 py-3 text-sm font-medium text-center text-gray-900 border border-gray-200 rounded-lg sm:w-auto hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 dark:text-white dark:border-gray-700 dark:hover:bg-gray-700 dark:focus:ring-gray-800">
                        // <svg class="w-4 h-4 mr-2 text-gray-500 dark:text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 496 512">
                    // </a> 
                    <a href="mailto:tatianagrigoriev7@gmail.com" class="inline-flex items-center justify-center w-full px-5 py-3 mb-2 mr-2 text-sm font-medium text-gray-900 bg-white border border-gray-200 rounded-lg sm:w-auto focus:outline-none hover:bg-gray-100 focus:z-10 focus:ring-4 focus:ring-gray-200 button-main">
                        {"Get in touch"}
                    </a>
                </div>
            </div>                
        </div>
    </section>
    }
}