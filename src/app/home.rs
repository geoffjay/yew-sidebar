use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::State;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let onclick = Dispatch::<State>::new().reduce_mut_callback(|state| state.open_sidebar());

    use_effect_with(
        (),
        move |_| {
            Dispatch::<State>::new().reduce_mut(|state| {
                state.set_sidebar_title("Home".to_string());
            });
            || {}
        },
    );

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
                <div class="flex flex-col items-center justify-center gap-4">
                    <p>
                        { "Content to demonstrate scrolling that doesn't have an affect on the open drawer." }
                    </p>
                    <div class="bg-emerald-50  w-[512px] h-64 border rounded-md border-slate-600 drop-shadow-lg"></div>
                    <div class="bg-emerald-100 w-[512px] h-64 border rounded-md border-slate-600 drop-shadow-lg"></div>
                    <div class="bg-emerald-200 w-[512px] h-64 border rounded-md border-slate-600 drop-shadow-lg"></div>
                    <div class="bg-emerald-300 w-[512px] h-64 border rounded-md border-slate-600 drop-shadow-lg"></div>
                    <div class="bg-emerald-400 w-[512px] h-64 border rounded-md border-slate-600 drop-shadow-lg"></div>
                    <div class="bg-emerald-500 w-[512px] h-64 border rounded-md border-slate-600 drop-shadow-lg"></div>
                    <div class="bg-emerald-600 w-[512px] h-64 border rounded-md border-slate-600 drop-shadow-lg"></div>
                    <div class="bg-emerald-700 w-[512px] h-64 border rounded-md border-slate-600 drop-shadow-lg"></div>
                    <div class="bg-emerald-800 w-[512px] h-64 border rounded-md border-slate-600 drop-shadow-lg"></div>
                    <div class="bg-emerald-900 w-[512px] h-64 border rounded-md border-slate-600 drop-shadow-lg"></div>
                    <div class="bg-emerald-950 w-[512px] h-64 border rounded-md border-slate-600 drop-shadow-lg"></div>
                </div>
            </header>
        </div>
    }
}
