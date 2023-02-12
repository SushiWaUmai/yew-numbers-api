use yew::prelude::*;

use crate::components::{header::Header, footer::Footer};

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class={classes!("absolute", "inset-0", "flex", "flex-col", "text-white")}>
            <Header />
            <main class={classes!("flex-grow")}>{props.children.clone()}</main>
            <Footer />
        </div>
    }
}
