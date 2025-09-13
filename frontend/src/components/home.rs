use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct HomeProps {

}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    html! {
        <div>
            <h1>{"Home"}</h1>
        </div>
    }
}