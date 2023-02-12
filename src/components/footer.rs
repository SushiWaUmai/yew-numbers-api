use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class={classes!("px-2")}>
            <div class={classes!("container", "mx-auto", "bg-jet", "px-4", "py-2", "rounded-t-lg")}>
                {"Yew Numbers API 2023"}
            </div>
        </footer>
    }
}
