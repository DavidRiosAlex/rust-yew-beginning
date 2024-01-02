mod root;
mod components;
mod pages;

fn main() {
    yew::Renderer::<root::app::App>::new().render();
}
