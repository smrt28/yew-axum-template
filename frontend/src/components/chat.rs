#![allow(dead_code)]

use yew::{functional::{
    UseReducerHandle
}, function_component,
          html, use_effect_with, use_state, Html, Properties, 
          Reducible, Callback, use_node_ref, hook
};
use wasm_bindgen_futures::spawn_local;
use log::info;
use std::rc::Rc;
use web_sys::{WebSocket, MessageEvent, CloseEvent};
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



// Custom hook for WebSocket
#[hook]
fn use_websocket(url: &str, on_message: Callback<String>) -> (bool, Callback<String>) {
    let ws_ref = use_state(|| None::<WebSocket>);
    let connected = use_state(|| false);

    // Setup connection
    use_effect_with(url.to_string(), {
        let ws_ref = ws_ref.clone();
        let ws_ref2 = ws_ref.clone();
        let connected = connected.clone();
        let on_message = on_message.clone();

        move |url| {
            let url = url.clone();
            spawn_local(async move {
                if let Ok(ws) = WebSocket::new(&url) {
                    // Simple handler setup
                    let connected_clone = connected.clone();
                    let onopen: Closure<dyn FnMut(JsValue)>  = Closure::new(move |_| {
                        connected_clone.set(true)
                    });
                    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
                    onopen.forget();

                    let on_message_clone = on_message.clone();
                    let onmessage: Closure<dyn FnMut(MessageEvent)> = Closure::new(move |e: MessageEvent| {
                        if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                            on_message_clone.emit(String::from(text));
                        }
                    });
                    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                    onmessage.forget();

                    let connected_clone = connected.clone();
                    let onclose: Closure<dyn FnMut(CloseEvent)> = Closure::new(move |_| connected_clone.set(false));
                    ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
                    onclose.forget();

                    ws_ref.set(Some(ws));
                }
            });


            move || {
                if let Some(ws) = (*ws_ref2).as_ref() {
                    let _ = ws.close();
                }
            }
        }
    });

    // Send message function
    let send_message = {
        let ws_ref = ws_ref.clone();
        Callback::from(move |message: String| {
            if let Some(ws) = (*ws_ref).as_ref() {
                let _ = ws.send_with_str(&message);
            }
        })
    };

    (*connected, send_message)
}


#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
    let _is_read_only = props.read_only;
    let on_ws_message = {
        let _state = props.state.clone();
        Callback::from(move |_message: String| {

        })
    };

    let (_connected, send_ws_message) = use_websocket("ws://localhost:3000/ws", on_ws_message);


    let textarea_ref = use_node_ref();

    let on_send = {
        let textarea_ref = textarea_ref.clone();
        let _state = props.state.clone();
        Callback::from(move |_| {
            if let Some(textarea) = textarea_ref.cast::<web_sys::HtmlTextAreaElement>() {
                let value = textarea.value();
                textarea.set_value("");
                info!("Send: {}", &value);
                send_ws_message.emit(value);
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
