use yew::prelude::*;

#[function_component]
pub fn AboutMe() -> Html{
    html!{
        <div class="w-full flex justify-center" id="about">
        <div class="max-w-screen-lg px-8 py-4 mt-16 rounded-lg shadow-lg bg-slate-50">
    <div class="flex justify-start -mt-16">
        <img class="object-cover w-20 h-20 border-2 rounded-full" alt="About me Avatar" src="/img/poza.webp"/>
    </div>

    <h2 class="flex justify-end mt-2 text-xl font-bold text-accent md:mt-0 font-mono">{"About"}</h2>

    <p class="mt-2 text-m text-main font-semibold font-mono">{"
    I am self-driven and passionate about the work I do.
I would be glad to help you create amazing content, regardless of whether if it's written or media."} <br/>
{"I don't settle for less than amazing results and am constantly seeking out ways I can improve myself and my abilities.
I have written multiple educational and informational covers as well as research papers."}
<br/>
{"I possess extensive experience in structuring and optimizing the layout of power-point presentations."}</p>

    <div class="flex justify-end mt-4">
        <a href="#" class="text-xl font-mono font-bold text-accent" tabindex="0" role="link">{"Tatiana Grigoriev"}</a>
    </div>
</div>
</div>
    }
}

#[function_component]
pub fn ContactInfo() -> Html {
    html! {
        <div id="contact" class="w-full flex justify-center my-10">
        <div class="max-w-screen-lg w-full overflow-hidden bg-slate-50 rounded-lg shadow-lg flex justify-between">
        <div class="px-6 py-4 flex-1">
            <h1 class="text-2xl font-semibold text-main font-mono">{"Get in touch"}</h1>
            <div class="flex items-center mt-4 text-main">
                <svg aria-label="email icon" class="w-6 h-6 fill-current" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path fill-rule="evenodd" clip-rule="evenodd" d="M3.00977 5.83789C3.00977 5.28561 3.45748 4.83789 4.00977 4.83789H20C20.5523 4.83789 21 5.28561 21 5.83789V17.1621C21 18.2667 20.1046 19.1621 19 19.1621H5C3.89543 19.1621 3 18.2667 3 17.1621V6.16211C3 6.11449 3.00333 6.06765 3.00977 6.0218V5.83789ZM5 8.06165V17.1621H19V8.06199L14.1215 12.9405C12.9499 14.1121 11.0504 14.1121 9.87885 12.9405L5 8.06165ZM6.57232 6.80554H17.428L12.7073 11.5263C12.3168 11.9168 11.6836 11.9168 11.2931 11.5263L6.57232 6.80554Z"/>
                </svg>
                <a class="px-2 text-lg font-mono" href="mailto:tatianagrigoriev7@gmail.com?Subject=Need a presentation">{"tatianagrigoriev7@gmail.com"}</a>
            </div>
            <div class="flex items-center mt-4 text-main">
                <svg xmlns="http://www.w3.org/2000/svg"  viewBox="0 0 24 24" class="w-6 h-6 fill-current">
                    <path d="M 20.572266 3.0117188 C 20.239891 2.9764687 19.878625 3.028375 19.515625 3.171875 C 19.065625 3.348875 12.014406 6.3150313 5.4414062 9.0820312 L 3.2695312 9.9960938 C 2.4285313 10.337094 2.0039062 10.891672 2.0039062 11.638672 C 2.0039062 12.161672 2.22525 12.871063 3.28125 13.289062 L 6.9472656 14.757812 C 7.2642656 15.708813 8.0005469 17.916906 8.1855469 18.503906 C 8.2955469 18.851906 8.5733906 19.728594 9.2753906 19.933594 C 9.4193906 19.982594 9.5696563 20.007813 9.7226562 20.007812 C 10.165656 20.007812 10.484625 19.801641 10.640625 19.681641 L 12.970703 17.710938 L 15.800781 20.328125 C 15.909781 20.439125 16.486719 21 17.261719 21 C 18.228719 21 18.962234 20.195016 19.115234 19.416016 C 19.198234 18.989016 21.927734 5.2870625 21.927734 5.2890625 C 22.172734 4.1900625 21.732219 3.6199531 21.449219 3.3769531 C 21.206719 3.1694531 20.904641 3.0469688 20.572266 3.0117188 z M 19.910156 5.171875 C 19.533156 7.061875 17.478016 17.378234 17.166016 18.865234 L 13.029297 15.039062 L 10.222656 17.416016 L 11 14.375 C 11 14.375 16.362547 8.9468594 16.685547 8.6308594 C 16.945547 8.3778594 17 8.2891719 17 8.2011719 C 17 8.0841719 16.939781 8 16.800781 8 C 16.675781 8 16.506016 8.1197812 16.416016 8.1757812 C 15.272669 8.8885973 10.404094 11.662239 8.0078125 13.025391 L 4.53125 11.636719 L 6.21875 10.927734 C 10.51775 9.1177344 18.174156 5.893875 19.910156 5.171875 z"/>
                </svg>
                <a class="px-2 text-lg text-main font-mono" href="https://t.me/Marcelline34"  target="_blank" >{"t.me/Marcelline34"}</a>
            </div>
        </div>
        <img class="object-cover object-center w-1/2 h-72 justify-end" src="/img/contact.svg" alt="avatar"/>
    </div>
        </div>
    }
}


#[function_component]
pub fn Contact() -> Html {
    html!{
        <section>
            <AboutMe/>
            <ContactInfo/>
        </section>
    }
}

#[function_component]
pub fn Footer() -> Html {
    html! {
        <footer class="bg-slate-50">
    <div class="container flex flex-col items-center justify-between p-6 mx-auto space-y-4 sm:space-y-0 sm:flex-row">
        <p class="text-sm text-main">{"© Copyright 2023. All Rights Reserved."}</p>
        <a href="https://github.com/mtricolici98/" class="text-sm text-main">{"Created"}<p style="display: none;">{"with love"}</p>{" by mtricolici"}</a>
        <a href="https://storyset.com/online" class="text-sm text-main">{"Online illustrations by Storyset"}</a>
    </div>
</footer>        
    }
}


#[function_component]
pub fn Pricing() -> Html{
    html! {
        <section class="bg-white">
        <div class="max-w-screen-xl px-4 py-8 mx-auto lg:py-24 lg:px-6">
            <div class="max-w-screen-md mx-auto mb-8 text-center lg:mb-12">
                <h2 class="mb-4 text-3xl font-extrabold tracking-tight text-gray-900">{"Designed for business teams like yours"}</h2>
                <p class="mb-5 font-light text-gray-500 sm:text-xl">{"Something something something, pizdet pizda dada "}</p>
            </div>
            <div class="space-y-8 lg:grid lg:grid-cols-3 sm:gap-6 xl:gap-10 lg:space-y-0">
                <div class="flex flex-col max-w-lg p-6 mx-auto text-center text-gray-900 bg-white border border-gray-100 rounded-lg shadow xl:p-8">
                    <h3 class="mb-4 text-2xl font-semibold">{"Enchanced"}</h3>
                    <p class="font-light text-gray-500 sm:text-lg">{"Best for short pitches."}</p>
                    <div class="flex items-baseline justify-center my-8">
                        <span class="mr-2 text-5xl font-extrabold">{"$15"}</span>
                        <span class="text-gray-500">{"1$/ Slide"}</span>
                    </div>
                    <ul role="list" class="mb-8 space-y-4 text-left">
                        <li class="flex items-center space-x-3">
                            <svg class="flex-shrink-0 w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path></svg>
                            <span>{"Simple but elegant animations"}</span>
                        </li>
                        <li class="flex items-center space-x-3">
                            <svg class="flex-shrink-0 w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path></svg>
                            <span><span class="font-semiblod">{"15"}</span> {" Slides included"}</span>
                        </li>
                    </ul>
                    <a href="#" class="text-white bg-purple-600 hover:bg-purple-700 focus:ring-4 focus:ring-purple-200 font-medium rounded-lg text-sm px-5 py-2.5 text-center">{"Get a quote"}</a>
                </div>
                <div class="flex flex-col max-w-lg p-6 mx-auto text-center text-gray-900 bg-white border border-gray-100 rounded-lg shadow xl:p-8">
                    <h3 class="mb-4 text-2xl font-semibold">{"Amazing"}</h3>
                    <p class="font-light text-gray-500 sm:text-lg">{"Best for a stunning event."}</p>
                    <div class="flex items-baseline justify-center my-8">
                        <span class="mr-2 text-5xl font-extrabold">{"$25"}</span>
                        <span class="text-gray-500">{"1.5$/ Slide"}</span>
                    </div>
                    <ul role="list" class="mb-8 space-y-4 text-left">
                        <li class="flex items-center space-x-3">
                            <svg class="flex-shrink-0 w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path></svg>
                            <span>{"Simple but elegant animations"}</span>
                        </li>
                        <li class="flex items-center space-x-3">
                            <svg class="flex-shrink-0 w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path></svg>
                            <span><span class="font-semiblod">{"20"}</span> {" Slides included"}</span>
                        </li>
                    </ul>
                    <a href="#" class="text-white bg-purple-600 hover:bg-purple-700 focus:ring-4 focus:ring-purple-200 font-medium rounded-lg text-sm px-5 py-2.5 text-center">{"Get a quote"}</a>
                </div>
                <div class="flex flex-col max-w-lg p-6 mx-auto text-center text-gray-900 bg-white border border-gray-100 rounded-lg shadow xl:p-8">
                    <h3 class="mb-4 text-2xl font-semibold">{"Majestic"}</h3>
                    <p class="font-light text-gray-500 sm:text-lg">{"Functionality that would leave people guessing."}</p>
                    <div class="flex items-baseline justify-center my-8">
                        <span class="mr-2 text-5xl font-extrabold">{"$50"}</span>
                        <span class="text-gray-500">{"2.5$/ Slide"}</span>
                    </div>
                    <ul role="list" class="mb-8 space-y-4 text-left">
                        <li class="flex items-center space-x-3">
                            <svg class="flex-shrink-0 w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path></svg>
                            <span>{"Simple but elegant animations"}</span>
                        </li>
                        <li class="flex items-center space-x-3">
                            <svg class="flex-shrink-0 w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path></svg>
                            <span><span class="font-semiblod">{"25"}</span> {" Slides included"}</span>
                        </li>
                    </ul>
                    <a href="#" class="text-white bg-purple-600 hover:bg-purple-700 focus:ring-4 focus:ring-purple-200 font-medium rounded-lg text-sm px-5 py-2.5 text-center">{"Get a quote"}</a>
                </div>
           </div>
        </div>
      </section>
    }
}
