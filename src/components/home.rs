use yew::prelude::*;

#[function_component]
pub fn Home() -> Html { 
    html !{ 
        <section>
            <div class="grid max-w-screen-xl px-4 pt-10 pb-8 mx-auto lg:gap-8 xl:gap-0 lg:grid-cols-12 lg:pt-16">
                <div class="hidden lg:mt-0 lg:col-span-7 lg:flex">
                    <img class="" src="img/graphic-main.webp" alt="Main image"/>
                </div>
                <div class="mr-auto place-self-center lg:col-span-5">
                    <h1 class="max-w-2xl mb-4 text-4xl font-extrabold leading-none tracking-tight md:text-5xl xl:text-6xl text-accent">{"Turn your"} <br/> {"presentation"} <br/> {"into an event."} </h1>
                    <p class="max-w-2xl mb-6 font-light lg:mb-8 md:text-lg lg:text-xl text-main">{"PowerPoint presentations are often the backbone of any business presentation, whether it is a sales pitch, a training session or a board meeting. The design of the presentation can make or break the delivery of the message. A visually appealing and professional presentation can set the tone for the presentation and engage the audience, making it easier for the speaker to deliver their message. On the other hand, a poorly designed presentation can leave the audience feeling disinterested and unimpressed."}</p>
                    <div class="space-y-4 sm:flex sm:space-y-0 sm:space-x-4">
                        // <a href="https://github.com/themesberg/landwind" class="inline-flex items-center justify-center w-full px-5 py-3 text-sm font-medium text-center text-gray-900 border border-gray-200 rounded-lg sm:w-auto hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 dark:text-white dark:border-gray-700 dark:hover:bg-gray-700 dark:focus:ring-gray-800">
                            // <svg class="w-4 h-4 mr-2 text-gray-500 dark:text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 496 512">
                        // </a> 
                        <a href="/gallery" class="inline-flex items-center justify-center w-full px-5 py-3 mb-2 mr-2 text-sm font-medium text-gray-900 bg-white border border-gray-200 rounded-lg sm:w-auto focus:outline-none hover:bg-gray-100 focus:z-10 focus:ring-4 focus:ring-gray-200 button-main">
                            {"See examples"}
                        </a>
                    </div>
                </div>                
            </div>
        </section>
    }
}