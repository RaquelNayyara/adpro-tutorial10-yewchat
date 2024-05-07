use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-gradient-to-r from-blue-500 to-purple-600 flex w-screen h-screen items-center justify-center">
            <div class="flex flex-col bg-white shadow-lg p-10 rounded-lg">
                <h2 class="text-2xl font-bold mb-6 text-gray-700">
                    <span class="text-purple-600">{"Yew"}</span><span>{"Chat"}</span>
                </h2>
                <form class="flex">
                    <input {oninput} class="flex-1 rounded-l-lg p-4 border-gray-300 focus:outline-none focus:ring-2 focus:ring-purple-500 transition duration-200" placeholder="Username"/>
                    <Link<Route> to={Route::Chat}>
                        <button {onclick} disabled={username.len()<1} class="px-8 rounded-r-lg bg-purple-600 hover:bg-purple-700 text-white font-bold p-4 uppercase transition duration-200">{"Go Chatting!"}</button>
                    </Link<Route>>
                </form>
                <p class="mt-4 text-gray-600 text-sm">{"Enter your username to start chatting."}</p>
            </div>
        </div>
    }
}