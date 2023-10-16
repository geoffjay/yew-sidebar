use yew::prelude::*;

/// Sidebar component
#[function_component(Sidebar)]
pub fn sidebar() -> Html {
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
        // "translate-x-full",
        "z-20",
        "p-4",
    ];

    html! {
        <aside class={classes!(sidebar_classes)} tabindex="-1" aria-labelledby="drawer">
            <p class="text-bold text-2xl">{"Sidebar"}</p>
        </aside>
    }
}
