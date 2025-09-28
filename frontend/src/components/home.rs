use yew::{function_component, html, use_effect_with, use_reducer, use_state, Html, Properties, UseStateHandle};
use wasm_bindgen_futures::spawn_local;
use log::info;
use gloo_timers::future::TimeoutFuture;

use crate::components::login::*;
use crate::components::chat::*;
use crate::components::simcalendar::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub title: String,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    let counter: UseStateHandle<i32> = use_state(|| 0);
    let counter_to_increment = counter.clone();

    let chat_state1 = use_reducer(ChatState::default);
    let chat_state2 = use_reducer(ChatState::default);
    let chat_state3 = use_reducer(ChatState::default);

    use_effect_with(*counter, move |_| {
        spawn_local(async move {
            TimeoutFuture::new(1000).await;
            counter_to_increment.set(*counter_to_increment + 1);
            info!("Home");
        });
    });


    let sim_calendar_state = use_reducer(SimCalendarState::default);

    html! {
        <div class="">

        <div class="home">
        <Login/>
        </div>

/*
           <SimCalendar state={sim_calendar_state}/>



            <div class="chats">
                <Chat name="Tom" state={chat_state1} read_only=false/>
                <Chat name="Dick" state={chat_state2} read_only=true/>
                <Chat name="Harry" state={chat_state3} read_only=true/>
            </div>
*/


        </div>

    }
}