use gloo::net::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct FactProps {
    pub number: i64,
}

#[function_component(Fact)]
pub fn fact(props: &FactProps) -> Html {
    let num_fact = use_state(|| "".to_owned());
    let number = props.number;
    let num_input = use_state(|| number);

    {
        let num_fact = num_fact.clone();
        let num_input = num_input.clone();

        use_effect_with_deps(
            move |_| {
                let num_fact = num_fact.clone();

                let num_input = num_input.clone();
                num_input.set(number);

                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_fact = Request::get(
                        format!(
                            "https://cors-anywhere.herokuapp.com/http://numbersapi.com/{}",
                            number
                        )
                        .as_str(),
                    )
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                    num_fact.set(fetched_fact);
                });
            },
            number,
        )
    }

    let navigator = use_navigator().unwrap();
    let num_input_node = use_node_ref();

    let handle_submit = {
        let num_input_node = num_input_node.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            num_input_node.cast::<HtmlInputElement>().unwrap();
            if let Some(input) = num_input_node.cast::<HtmlInputElement>() {
                input.blur().unwrap_or_else(|_| {});
                let value: i64 = input.value().parse().unwrap_or_else(|_| 1);
                navigator.push(&Route::Fact { number: value })
            }
        })
    };

    let handle_change = {
        let num_input = num_input.clone();
        let num_input_node = num_input_node.clone();

        Callback::from(move |_| {
            if let Some(input) = num_input_node.cast::<HtmlInputElement>() {
                let value: i64 = input.value().parse().unwrap_or_else(|_| 1);
                num_input.set(value);
            }
        })
    };

    html! {
        <div class={classes!("h-full")}>
            <div class={classes!("container", "mx-auto", "h-full", "flex", "flex-col")}>
                <div class={classes!("flex-grow", "grid", "place-items-center", "h-full", "p-2")}>
                    <div>
                        <form onsubmit={handle_submit}>
                            <input
                                type="number"
                                ref={num_input_node}
                                value={num_input.to_string()}
                                onchange={handle_change}
                                class={classes!("inline", "text-8xl", "text-tiffany", "text-center", "bg-night", "outline-none", "w-full")}
                            />
                            <input type="submit" class={classes!("hidden")} />
                        </form>
                        <p class={classes!("text-2xl", "text-center")}>{num_fact.as_str()}</p>
                    </div>
                </div>
                <div class={classes!("flex", "justify-center", "gap-x-4", "m-8")}>
                    <Link<Route> to={Route::Fact { number: number - 1 }}>
                        <span class={classes!("text-2xl", "bg-jet", "px-4", "py-2", "rounded", "hover:bg-persian", "duration-200")}>
                            {"<"}
                        </span>
                    </Link<Route>>
                    <Link<Route> to={Route::Fact { number: number + 1 }}>
                        <span class={classes!("text-2xl", "bg-jet", "px-4", "py-2", "rounded", "hover:bg-persian", "duration-200")}>
                            {">"}
                        </span>
                    </Link<Route>>
                </div>
            </div>
        </div>
    }
}
