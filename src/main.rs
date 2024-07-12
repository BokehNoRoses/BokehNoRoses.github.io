use bokehnoroses::components::{Carousel, CertPanel, Terminal};
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Terminal />
                <CertPanel />
                <Carousel />
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
