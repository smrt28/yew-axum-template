use yew::{function_component, html, use_effect_with, use_state, Html, Properties, UseStateHandle};
use wasm_bindgen_futures::spawn_local;
use log::info;
use gloo_timers::future::TimeoutFuture;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub title: String,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    let counter: UseStateHandle<i32> = use_state(|| 0);
    let counter_to_increment = counter.clone();
    use_effect_with(*counter, move |_| {
        spawn_local(async move {
            TimeoutFuture::new(1000).await;
            counter_to_increment.set(*counter_to_increment + 1);
            info!("Home");
        });
    });

    html! {
        <div>
            <h1>{"Counter: "}{*counter}</h1>
            <h2>{props.title.clone()}</h2>
        </div>
    }
}