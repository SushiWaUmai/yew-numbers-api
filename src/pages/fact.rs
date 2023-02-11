use gloo::net::http::Request;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FactProps {
    pub number: i32,
}

#[function_component(Fact)]
pub fn fact(props: &FactProps) -> Html {
    let num_fact = use_state(|| "".to_owned());
    {
        let num_fact = num_fact.clone();
        let number = props.number;

        use_effect_with_deps(
            move |_| {
                let num_fact = num_fact.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_fact =
                        Request::get(format!("http://numbersapi.com/{}", number).as_str())
                            .send()
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap();
                    num_fact.set(fetched_fact);
                });
            },
            (),
        )
    }

    html! {
        <h1>{num_fact.as_str()}</h1>
    }
}
