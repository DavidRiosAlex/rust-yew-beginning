use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{switch, Routes};

const STYLE_FILE: &str = include_str!("app.scss");

#[function_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    html! {
        <main class={stylesheet}>
            <BrowserRouter>
                <Switch<Routes> render={switch}/>
            </BrowserRouter>
        </main>
    }
}
