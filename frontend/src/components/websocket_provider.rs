#![allow(dead_code)]
use yew::prelude::*;
use web_sys::{WebSocket, MessageEvent, CloseEvent};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use log::info;
use std::collections::HashMap;
//use std::rc::Rc;

#[derive(Clone)]
pub struct WebSocketContext {
    pub connected: bool,
    pub send_message: Option<Callback<String>>,
    pub register_handler: Callback<(String, Callback<String>)>,
    pub unregister_handler: Callback<String>,
}

impl PartialEq for WebSocketContext {
    fn eq(&self, other: &Self) -> bool {
        self.connected == other.connected
    }
}

#[derive(Properties, PartialEq)]
pub struct WebSocketProviderProps {
    pub children: Children,
    pub url: String,
}

#[function_component(WebSocketProvider)]
pub fn websocket_provider(props: &WebSocketProviderProps) -> Html {
    let connected = use_state(|| false);
    let message_handlers = use_state(|| HashMap::<String, Callback<String>>::new());
    let send_message = use_state(|| None::<Callback<String>>);
    
    // Setup WebSocket connection
    use_effect_with(props.url.clone(), {
        let connected = connected.clone();
        let message_handlers = message_handlers.clone();
        let send_message = send_message.clone();
        let connected2 = connected.clone();
        let send_message2 = send_message.clone();

        move |url| {
            let url = url.clone();
            let connected = connected.clone();
            let message_handlers = message_handlers.clone();
            let send_message = send_message.clone();

            
            spawn_local(async move {
                if let Ok(ws) = WebSocket::new(&url) {
                    info!("Connecting to WebSocket: {}", url);
                    
                    // On open
                    let connected_clone = connected.clone();
                    let send_message_clone = send_message.clone();
                    let ws_clone = ws.clone();
                    let onopen: Closure<dyn FnMut(JsValue)> = Closure::new(move |_| {
                        info!("WebSocket connected");
                        connected_clone.set(true);
                        
                        // Create send callback
                        let ws_ref = ws_clone.clone();
                        let send_callback = Callback::from(move |message: String| {
                            if let Err(e) = ws_ref.send_with_str(&message) {
                                info!("Failed to send message: {:?}", e);
                            }
                        });
                        send_message_clone.set(Some(send_callback));
                    });
                    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
                    onopen.forget();

                    // On message - distribute to all registered handlers
                    let message_handlers_clone = message_handlers.clone();
                    let onmessage: Closure<dyn FnMut(MessageEvent)> = Closure::new(move |e: MessageEvent| {
                        if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                            let message = String::from(text);
                            info!("Received message: {}", message);
                            
                            // Send to all registered handlers
                            for handler in (*message_handlers_clone).values() {
                                handler.emit(message.clone());
                            }
                        }
                    });
                    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                    onmessage.forget();

                    // On close
                    let connected_clone = connected.clone();
                    let send_message_clone = send_message.clone();
                    let onclose: Closure<dyn FnMut(CloseEvent)> = Closure::new(move |_| {
                        info!("WebSocket disconnected");
                        connected_clone.set(false);
                        send_message_clone.set(None);
                    });
                    ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
                    onclose.forget();
                }
            });
            
            // Cleanup
            move || {
                connected2.set(false);
                send_message2.set(None);
            }
        }
    });

    // Create register/unregister callbacks
    let register_handler = {
        let message_handlers = message_handlers.clone();
        Callback::from(move |(id, handler): (String, Callback<String>)| {
            let mut handlers = (*message_handlers).clone();
            handlers.insert(id, handler);
            message_handlers.set(handlers);
        })
    };

    let unregister_handler = {
        let message_handlers = message_handlers.clone();
        Callback::from(move |id: String| {
            let mut handlers = (*message_handlers).clone();
            handlers.remove(&id);
            message_handlers.set(handlers);
        })
    };

    let context = WebSocketContext {
        connected: *connected,
        send_message: (*send_message).clone(),
        register_handler,
        unregister_handler,
    };

    html! {
        <ContextProvider<WebSocketContext> context={context}>
            {for props.children.iter()}
        </ContextProvider<WebSocketContext>>
    }
}

// Hook to use WebSocket in components
#[hook]
pub fn use_websocket_context(handler_id: String, on_message: Callback<String>) -> (bool, Option<Callback<String>>) {
    let context = use_context::<WebSocketContext>()
        .expect("WebSocketProvider not found");
    
    // Register message handler
    use_effect_with((handler_id.clone(), on_message.clone()), {
        let register = context.register_handler.clone();
        let unregister = context.unregister_handler.clone();
        
        move |(id, handler)| {
            register.emit((id.clone(), handler.clone()));
            
            // Cleanup
            let unregister = unregister.clone();
            let id = id.clone();
            move || {
                unregister.emit(id);
            }
        }
    });
    
    (context.connected, context.send_message.clone())
}