use yew::prelude::*;

#[function_component(NotFound)]
pub fn notfound() -> Html {
    html! {
        <h1 class={classes!("text-8xl")}>{"Not Found"}</h1>
    }
}
