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

fn get_action_class(action_code: i32) -> String {
    format!("action-{}", action_code)
}

impl Default for SimCalendarState {
    fn default() -> Self {
        Self {
            action_codes: [0; 7 * 24]
        }
    }
}

struct CalendarAction {
    index: usize,
    action_code: i32,
}

pub enum SimCalendarAction {
    SetAction(CalendarAction),
}

#[derive(Properties, PartialEq)]
pub struct SimCalendarProps {
    pub state:  UseReducerHandle<SimCalendarState>,
}

impl Reducible for SimCalendarState {
    type Action = SimCalendarAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self>
    {
        match action {
            SimCalendarAction::SetAction(action) => {
                let mut state = (*self).clone();
                state.action_codes[action.index] = action.action_code;
                Rc::new(state)
            }
        }
    }
}


#[function_component(SimCalendar)]
pub fn chat(props: &SimCalendarProps) -> Html {
    let days = vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    html! {
        <div class="calendar-container">
            <div class="calendar-header">
                <div class="hour-label-header"></div>
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
                            {for (0..7).map(|day| {
                                let index = hour * 7 + day;
                                let action_code = props.state.action_codes[index];
                                let class_name = format!("time-slot {}", get_action_class(action_code));
                                html! {
                                    <div class={class_name}></div>
                                }
                            })}
                        </div>
                    }
                })}
            </div>
        </div>
    }
}