use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class={classes!("bg-neutral-900", "p-2")}>
            <div class={classes!("container", "mx-auto")}>{"Yew Numbers API"}</div>
        </footer>
    }
}
