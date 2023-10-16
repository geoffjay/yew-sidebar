use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::State;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let onclick = Dispatch::<State>::new().reduce_mut_callback(|state| state.open_sidebar());

    html! {
        <div class="container text-center">
            <header class="space-y-8">
                <p class="space-x-4">
                    <button
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-emerald-600 text-slate-100 hover:bg-emerald-600/90"
                        {onclick}
                    >
                        { "Reveal" }
                    </button>
                </p>
                <div class="flex flex-col items-center justify-center">
                    <p class="w-64 h-64 border border-slate-200">{"stuff"}</p>
                    <p class="w-64 h-64 border border-slate-200">{"stuff"}</p>
                    <p class="w-64 h-64 border border-slate-200">{"stuff"}</p>
                    <p class="w-64 h-64 border border-slate-200">{"stuff"}</p>
                    <p class="w-64 h-64 border border-slate-200">{"stuff"}</p>
                </div>
            </header>
        </div>
    }
}
