use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(NotFound)]
pub fn notfound() -> Html {
    html! {
        <div class={classes!("h-full", "grid", "place-items-center")}>
            <div class={classes!("flex", "flex-col", "gap-y-8", "text-center")}>
                <h1 class={classes!("text-8xl")}>{"404 Not Found"}</h1>
                <Link<Route> to={Route::Home}>
                    <span class={classes!("text-2xl", "bg-persian", "px-4", "py-2", "rounded")}>
                        {"Back to Home"}
                    </span>
                </Link<Route>>
            </div>
        </div>
    }
}
