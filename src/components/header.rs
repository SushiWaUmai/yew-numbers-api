use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class={classes!("bg-neutral-800", "p-2")}>
            <div class={classes!("container", "mx-auto", "flex", "justify-between")}>
                <span>
                    <a href="/">{"Yew Numbers API"}</a>
                </span>
                <nav>
                    <ul class={classes!("flex")}>
                        <li>
                            <a href="/">{"Home"}</a>
                        </li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}
