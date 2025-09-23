use yew::{functional::{
    UseReducerHandle
}, function_component, html, use_effect_with, use_state, Html, Properties, UseStateHandle, Reducible, Callback, use_node_ref};
use wasm_bindgen_futures::spawn_local;
use log::info;
use gloo_timers::future::TimeoutFuture;
use std::rc::Rc;
use std::borrow::BorrowMut;
use serde_json::Number;
use crate::components::chat::ChatAction;

#[derive(Clone, PartialEq)]
pub struct SimCalendarState {
    action_codes: [i32; 7 * 24],
}


impl Default for SimCalendarState {
    fn default() -> Self {
        Self {
            action_codes: [0; 7 * 24]
        }
    }
}


pub enum SimCalendarAction {
    SetAction(Number, String),
}

#[derive(Properties, PartialEq)]
pub struct SimCalendarProps {
    pub state:  UseReducerHandle<SimCalendarState>,
}


impl Reducible for SimCalendarState {
    type Action = SimCalendarAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Rc::new((*self).clone())
    }
}


#[function_component(SimCalendar)]
pub fn chat(props: &SimCalendarProps) -> Html {
    let days = vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];


    html! {
        <div class="calendar-container">
            // Header with day names
            <div class="calendar-header">
                <div class="hour-label-header"></div> // Empty space above hour labels
                {for days.iter().map(|day| html! {
                    <div class="day-header">{day}</div>
                })}
            </div>

            // Time grid
            <div class="calendar-grid">
                {for (0..24).map(|hour| {
                    html! {
                        <div class="hour-row">
                            <div class="hour-label">{format!("{:02}:00", hour)}</div>
                            {for (0..7).map(|_day| html! {
                                <div class="time-slot"></div>
                            })}
                        </div>
                    }
                })}
            </div>
        </div>
    }
}