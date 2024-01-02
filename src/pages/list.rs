use yew::prelude::*;
use crate::components::typography::Typography;

#[function_component(List)]
pub fn list () -> Html {
    html! {
        <div>
            <Typography fontsize={30}>{"List"}</Typography>
        </div>
    }
}
