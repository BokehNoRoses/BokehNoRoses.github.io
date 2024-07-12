use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
    pub class: String,
    pub id: String,
    pub onclick: Callback<()>,
    pub onmouseover: Callback<()>,
    pub onmouseleave: Callback<()>,
}

pub enum Msg {
    Clicked,
    MouseOver,
    MouseLeave,
}

// https://yew.rs/docs/advanced-topics/struct-components/callbacks
// https://dev.to/davidedelpapa/yew-tutorial-06-custom-custom-custom-4ml3
pub struct TermBtn;

impl Component for TermBtn {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => ctx.props().onclick.emit(()), // run the callback associated with onclick event
            Msg::MouseOver => ctx.props().onmouseover.emit(()),
            Msg::MouseLeave => ctx.props().onmouseleave.emit(()),
        }
        false // return that the DOM element should not updated and view should not be called again
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = ctx.props().title.clone();
        let class = ctx.props().class.clone();
        let id = ctx.props().id.clone();
        let onclick = ctx.link().callback(|_| Msg::Clicked);
        let onmouseover = ctx.link().callback(|_| Msg::MouseOver);
        let onmouseleave = ctx.link().callback(|_| Msg::MouseLeave);
        html! {
            <button {class} {id} {onclick} {onmouseover} {onmouseleave}><h4 class="text">{title}</h4></button>
        }
    }
}
