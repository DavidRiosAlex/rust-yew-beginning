use yew::prelude::*;
use crate::components::typography::Typography;

#[function_component(Home)]
pub fn home () -> Html {
    html! {
        <div>
            <Typography fontsize={30}>{"Home"}</Typography>
        </div>
    }
}
