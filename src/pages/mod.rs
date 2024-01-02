mod not_found;
mod list;
mod home;

use yew::{html, Html};
use yew_router::Routable;



#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/not-found")]
    NotFound,
    #[at("/")]
    Home,
    #[at("/list")]
    List
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::NotFound => html! { <not_found::NotFound></not_found::NotFound> },
        Routes::Home => html! { <home::Home></home::Home> },
        Routes::List => html! { <list::List></list::List> }
    }
}
