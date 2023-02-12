use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={classes!("p-2")}>
            <div class={classes!("container", "mx-auto")}>
                <div class={classes!("h-96", "flex", "flex-col", "justify-center", "p-8", "gap-y-4")}>
                    <h1 class={classes!("text-6xl", "text-tiffany")}>{"Yew Numbers API"}</h1>
                    <span class={classes!("text-2xl", "px-2")}>
                        {"A Frontend for the "}
                        <a href="http://numbersapi.com/" target="_blank" class={classes!("text-tiffany")}>{"Numbers API"}</a>
                        {" written in "}
                        <a href="https://www.rust-lang.org/" target="_blank" class={classes!("text-tiffany")}>{"Rust"}</a>
                        {" with "}
                        <a href="https://yew.rs/" target="_blank" class={classes!("text-tiffany")}>{"Yew.rs"}</a>
                    </span>
                    <div class={classes!("my-4", "p-2")}>
                        <Link<Route> to={Route::Fact { number: 1 }}>
                            <span class={classes!("text-xl", "py-2", "px-4", "bg-persian", "rounded")}>
                                {"Get Started!"}
                            </span>
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </div>
    }
}
