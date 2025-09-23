use yew::{functional::{
    UseReducerHandle
}, function_component, html, use_effect_with, use_state, Html, Properties, UseStateHandle, Reducible, Callback, use_node_ref};
use wasm_bindgen_futures::spawn_local;
use log::info;
use gloo_timers::future::TimeoutFuture;
use std::rc::Rc;
use std::borrow::BorrowMut;

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
    pub read_only: bool,
    pub state:  UseReducerHandle<ChatState>,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
    let textarea_ref = use_node_ref();

    let on_send = {
        let textarea_ref = textarea_ref.clone();
        let state = props.state.clone();
        Callback::from(move |_| {
            if let Some(textarea) = textarea_ref.cast::<web_sys::HtmlTextAreaElement>() {
                let value = textarea.value();
                textarea.set_value("");
                info!("Send: {}", value);
                state.dispatch(ChatAction::MessageSent(value));
            }
        })
    };

    html! {
        <div class="chat">
         <div class="chat-messages">
            {
                for props.state.get_messages().iter().map(|message| {
                    html! {<p>{message}</p> }
                })
            }
         </div>


            if !props.read_only {
                <div class="chat-input">
                    <textarea ref={textarea_ref} />
                </div>
                <button onclick={on_send}>{"Send"}</button>
                <p>{props.state.messages_count()}</p>
            }
        </div>
    }
}
