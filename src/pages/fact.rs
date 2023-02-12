use gloo::net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct FactProps {
    pub number: i32,
}

#[function_component(Fact)]
pub fn fact(props: &FactProps) -> Html {
    let num_fact = use_state(|| "".to_owned());
    let number = props.number;
    {
        let num_fact = num_fact.clone();

        use_effect_with_deps(
            move |_| {
                let num_fact = num_fact.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_fact =
                        Request::get(format!("https://corsanywhere.herokuapp.com/http://numbersapi.com/{}", number).as_str())
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

    html! {
        <div class={classes!("h-full")}>
            <div class={classes!("container", "mx-auto", "h-full", "flex", "flex-col")}>
                <div class={classes!("flex-grow", "grid", "place-items-center", "h-full")}>
                    <div>
                        <h1 class={classes!("text-8xl", "text-tiffany", "text-center")}>{number}</h1>
                        <p class={classes!("text-2xl", "text-center")}>{num_fact.as_str()}</p>
                    </div>
                </div>
                <div class={classes!("flex", "justify-center", "gap-x-4", "m-8")}>
                    <Link<Route> to={Route::Fact { number: number - 1 }}>
                        <span class={classes!("text-2xl", "bg-jet", "px-4", "py-2", "rounded")}>{"<"}</span>
                    </Link<Route>>
                    <Link<Route> to={Route::Fact { number: number + 1 }}>
                        <span class={classes!("text-2xl", "bg-jet", "px-4", "py-2", "rounded")}>{">"}</span>
                    </Link<Route>>
                </div>
            </div>
        </div>
    }
}
