use yew::{functional::{
    UseReducerHandle
}, function_component, html, use_effect_with, use_state, Html, Properties, UseStateHandle, Reducible, Callback, use_node_ref, props, use_state_eq};
use wasm_bindgen_futures::spawn_local;
use log::info;
use gloo_timers::future::TimeoutFuture;
use std::rc::Rc;
use std::borrow::BorrowMut;
use crate::components::home::_HomeProps::title;
use web_sys::{WebSocket, MessageEvent, ErrorEvent, CloseEvent};
use wasm_bindgen::{prelude::*, JsCast};


#[derive(Clone, PartialEq, Default)]
pub struct ChatState {
    messages: Vec<String>,
}

impl ChatState {
    fn add_message(&mut self, message: &str) {
        if !message.is_empty() {
            self.messages.push(message.to_string());
        }
    }

    fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }
}


pub enum ChatAction {
    MessageSent(String),
}

impl Reducible for ChatState {
    type Action = ChatAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ChatAction::MessageSent(message) => {
                let mut new_state = (*self).clone();
                new_state.add_message(&message);
                Rc::new(new_state)
            }
        }
    }
}

impl ChatState {
    fn messages_count(&self) -> String {
        self.messages.len().to_string()
    }
}


#[derive(Properties, PartialEq)]
pub struct ChatProps {
    pub name: String,
    pub read_only: bool,
    pub state:  UseReducerHandle<ChatState>,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
    let is_read_only = props.read_only;
    let ws_ref = use_state(|| None::<WebSocket>);

    use_effect_with((), {
        let ws_ref = ws_ref.clone();
        move |_| {
            if is_read_only {
                return;
            }
            spawn_local(async move {
                let window = web_sys::window().unwrap();
                let location = window.location();
                let protocol = if location.protocol().unwrap() == "https:" { "wss:" } else { "ws:" };
                let host = location.host().unwrap();
                let ws_url = format!("{}//{}:3000/ws", protocol,
                                     host.split(':').next().unwrap_or("localhost"));

                match WebSocket::new(&ws_url) {
                    Ok(ws) => {
                        let onopen_callback = {
                            Closure::wrap(Box::new(move |_| {
                                info!("WebSocket connection opened");
                            }) as Box<dyn FnMut(JsValue)>)
                        };
                        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
                        onopen_callback.forget();


                        ws_ref.set(Some(ws));
                    }
                    Err(e) => {
                        info!("WebSocket connection error: {:?}", e);
                    }
                }


                //let ws_url = format!("{}//{}:8080/ws", protocol, host.split(':').next().unwrap_or("localhost"));
                info!("protocol: {}", ws_url);
            });
        }
    });


    let textarea_ref = use_node_ref();

    let on_send = {
        let textarea_ref = textarea_ref.clone();
        let state = props.state.clone();
        Callback::from(move |_| {
            if let Some(textarea) = textarea_ref.cast::<web_sys::HtmlTextAreaElement>() {
                let value = textarea.value();
                textarea.set_value("");
                info!("Send: {}", value);

                if let Some(ws) = (*ws_ref).as_ref() {
                    if ws.ready_state() != WebSocket::OPEN {
                        return;
                    }

                    match ws.send_with_str(&value) {
                        Ok(_) => {
                            info!("Sent via WebSocket: {}", value);
                            state.dispatch(ChatAction::MessageSent(value));
                        }
                        Err(e) => {
                            info!("Failed to send message: {:?}", e);
                        }
                    }

                } else {
                    info!("WebSocket is not ready");
                }
            }
        })
    };

    html! {
        <div class="chat">
        <div class="chat-head">
        <h1> {props.name.clone()} </h1>
        </div>
         <div class="chat-messages">
            {
                for props.state.get_messages().iter().map(|message| {
                    html! {<div class="chat-message">{message}</div> }
                })
            }
         </div>


            if !props.read_only {
                <div class="chat-input">
                    <textarea ref={textarea_ref} />
                    <button onclick={on_send}>{"Send"}</button>
                </div>
            }
        </div>
    }
}
