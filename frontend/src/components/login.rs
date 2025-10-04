use std::slice::SliceIndex;
use log::info;
use web_sys::{HtmlInputElement, HtmlElement};
use gloo::net::http::Request;
use gloo_timers::callback::Timeout;
use serde_json::json;
use yew::{functional::{
    UseReducerHandle
}, function_component,
          html, use_effect_with, use_state, Html, Properties,
          UseStateHandle, Reducible, Callback, use_node_ref, props, use_state_eq, hook
};
use wasm_bindgen::JsCast;
use yew::platform::spawn_local;
use crate::fetch::*;

#[derive(Properties, PartialEq)]
pub struct LoginProps {

}


#[derive(serde::Deserialize, Debug)]
struct LoginResponse {
    status: String,
}


#[function_component(Login)]
pub fn chat(props: &LoginProps) -> Html {
    let is_register = use_state(|| false);
    let pw1 = use_node_ref();
    let pw2 = use_node_ref();
    let error_message = use_state(|| String::new());
    let username = use_node_ref();

    let on_change = {
        let is_register = is_register.clone();
        let error_message = error_message.clone();
        Callback::from(move |event: web_sys::Event| {
            let target = event.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            let is_checked = input.checked();
            is_register.set(is_checked);
            error_message.set(String::new());
            info!("Register checkbox: {}", is_checked);

        })
    };

    let on_register_or_login = {
        let pw1 = pw1.clone();
        let pw2 = pw2.clone();
        let username = username.clone();
        let error_message = error_message.clone();

        let is_register = is_register.clone();
        Callback::from(move |_| {
            let pw1_value = pw1.cast::<HtmlInputElement>().unwrap().value();

            if *is_register {
                let pw2_value = pw2.cast::<HtmlInputElement>().unwrap().value();
                info!("Register {} {}", pw1_value, pw2_value);
                if *pw1_value != *pw2_value {
                    error_message.set("Passwords do not match".into());
                    return;
                }

                if pw1_value.len() < 4 {
                    error_message.set("Password must be at least 4 characters".into());
                    return
                }

            }

            let login_name = username.cast::<HtmlInputElement>().unwrap().value();

            Fetch::new()
                .post()
                .url("/login")
                .data(&json!({
                        "username": login_name,
                        "password": pw1_value,
                    }))
                .fetch(|resp: FetchResponse<LoginResponse>| async move {
                    info!("Login response: {:?}", resp);
                });
        })
    };

    let error_message = error_message.clone();


    let button_label = if *is_register {
        "Register"
    } else {
        "Login"
    };

    html! {
        <div class="login-container">
        <div class="registration">
                if *is_register {
                     <div class="invitation-code">
                         {"Invitation code: "}
                         <input type="text" class="register-invitation"/>
                     </div>
                }

                <div class="registration-container">
                    <input id="checkbox" type="checkbox" onchange={on_change}/>
                    <label for="checkbox">{"Register"}</label>
                </div>
            </div>
        <div class="login">


            <label for="username">{"Name:"}</label>
            <input ref={username} type="text" class="login-name"/>


            <div class="login-password-entry">

            <div>
                <label for="password">{"Password:"}</label>
                <input ref={pw1} type="password" class="login-password"/>
            </div>

            if *is_register {
            <div>
                <label for="password">{"Retype password:"}</label>
                <input ref={pw2} type="password" class="login-password"/>
            </div>
            }
            </div>

            <button class="login-button" onclick={on_register_or_login}>
                {button_label}
            </button>
        </div>

        <div class="login-error-message">{(*error_message).clone()}</div>

        </div>

    }
}