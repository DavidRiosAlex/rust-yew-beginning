use yew::prelude::*;
use stylist::Style;

#[derive(Properties, PartialEq)]
pub struct TypographyProps {
    pub fontsize: i32,
    pub children: Html,
}

#[function_component(Typography)]
pub fn typography(props: &TypographyProps) -> Html {

    let stylesheet = Style::new(format!(r#"
        font-size: {}px;
    "#, props.fontsize)).unwrap();
    
    html!{
        <p class={stylesheet}>{props.children.clone()}</p>
    }
}