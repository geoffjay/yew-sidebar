use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::State;

/// About page
#[function_component(About)]
pub fn about() -> Html {
    let onclick = Dispatch::<State>::new().reduce_mut_callback(|state| state.open_sidebar());

    use_effect_with(
        (),
        move |_| {
            Dispatch::<State>::new().reduce_mut(|state| {
                state.set_sidebar_title("About".to_string());
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
            </header>
        </div>
    }
}
