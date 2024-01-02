use yew::prelude::*;
use crate::components::typography::Typography;

#[function_component(NotFound)]
pub fn not_found () -> Html {
    html! {
        <div>
            <Typography fontsize={30}>{"Not Found"}</Typography>
        </div>
    }
}
