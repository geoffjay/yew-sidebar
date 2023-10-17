use yew::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};

use crate::store::State;

/// Sidebar component
#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let is_open = use_selector(|state: &State| state.is_sidebar_open());
    let onclick = Dispatch::<State>::new().reduce_mut_callback(|state| state.close_sidebar());
    let title = use_selector(|state: &State| state.sidebar.title.clone());

    let sidebar_classes = vec![
        "h-screen",
        "w-[500px]",
        "flex",
        "flex-col",
        "drop-shadow-xl",
        "bg-slate-200",
        "border-l",
        "border-slate-400",
        "top-0",
        "right-0",
        "bottom-0",
        "fixed",
        "overscroll-y-scroll",
        "transition-transform",
        "z-20",
        "p-4",
    ];

    log::info!("is_open: {:?}", *is_open);

    html! {
        if *is_open {
            <aside class={classes!(sidebar_classes)} tabindex="-1">
                <div class="flex flex-row justify-between w-full">
                    <p class="text-bold text-2xl">{title}</p>
                    <button
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-2 py-2 bg-emerald-600 text-slate-100 hover:bg-emerald-600/90"
                        {onclick}
                    >
                        <Icon icon_id={IconId::FontAwesomeRegularCircleXmark} />
                    </button>
                </div>
            </aside>
        }
    }
}
