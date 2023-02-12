use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class={classes!("bg-neutral-800", "p-2")}>
            <div class={classes!("container", "mx-auto", "flex", "justify-between")}>
                <span>
                    <Link<Route> to={Route::Home}>{"Yew Numbers API"}</Link<Route>>
                </span>
                <nav>
                    <ul class={classes!("flex")}>
                        <li>
                            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                        </li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}
