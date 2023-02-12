use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class={classes!("p-4")}>
            <div class={classes!("container", "mx-auto", "flex", "justify-between", "bg-jet", "px-4", "py-2", "rounded-full")}>
                <span>
                    <Link<Route> to={Route::Home}>
                        {"Yew Numbers API"}
                    </Link<Route>>
                </span>
                <nav>
                    <ul class={classes!("flex", "gap-x-4")}>
                        <li>
                            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                        </li>
                        <li>
                            <a href="https://github.com/SushiWaUmai/yew-numbers-api" target="_blank">{"Github"}</a>
                        </li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}
