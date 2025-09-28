use std::slice::SliceIndex;
use log::info;
use web_sys::{HtmlInputElement, HtmlElement};

use yew::{functional::{
    UseReducerHandle
}, function_component,
          html, use_effect_with, use_state, Html, Properties,
          UseStateHandle, Reducible, Callback, use_node_ref, props, use_state_eq, hook
};
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct LoginProps {

}

#[function_component(Login)]
pub fn chat(props: &LoginProps) -> Html {
    let is_register = use_state(|| false);
    let pw1 = use_node_ref();
    let pw2 = use_node_ref();
    let error_message_button = use_node_ref();

    let on_change = {
        let is_register = is_register.clone();
        let error_message_button = error_message_button.clone();
        Callback::from(move |event: web_sys::Event| {
            let target = event.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            let is_checked = input.checked();
            is_register.set(is_checked);
            if let Some(el) = error_message_button.cast::<HtmlElement>() {
                el.set_inner_text("");
            }
            info!("Register checkbox: {}", is_checked);

        })
    };

    let on_register_or_login = {
        let pw1 = pw1.clone();
        let pw2 = pw2.clone();
        let error_message_button = error_message_button.clone();

        let is_register = is_register.clone();
        Callback::from(move |_| {
            if *is_register {
                let pw1_value = pw1.cast::<HtmlInputElement>().unwrap().value();
                info!("Register or login {}", pw1_value);
                if let Some(el) = error_message_button.cast::<HtmlElement>() {
                    el.set_inner_text(&pw1_value);
                }
            }
        })
    };


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
                    <input type="checkbox" onchange={on_change}/>
                    <label for="checkbox">{"Register"}</label>
                </div>
            </div>
        <div class="login">


            <label for="username">{"Name:"}</label>
            <input type="text" class="login-name"/>

            <label for="password">{"Password:"}</label>
            <input ref={pw1} type="password" class="login-password"/>

            if *is_register {
            <label for="password">{"Retype password:"}</label>
            <input ref={pw2} type="password" class="login-password"/>
            }
            <button class="login-button" onclick={on_register_or_login} >{button_label}</button>

            <div class="login-error" ref={error_message_button}></div>

        </div>
        </div>

    }
}