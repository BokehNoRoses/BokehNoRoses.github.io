use super::termbtn::TermBtn;
use yew::prelude::*;

// https://github.com/yewstack/yew/blob/master/examples/inner_html
const COWSAY: &str = include_str!("../../assets/excerpts/COWSAY.html");
const P_INTRO: &str = include_str!("../../assets/excerpts/P_INTRO.html");
const ACCOMP: &str = include_str!("../../assets/excerpts/ACCOMP.html");
const U_SCRIPTS: &str = include_str!("../../assets/excerpts/U_SCRIPTS.html");
const WIP: &str = include_str!("../../assets/excerpts/WIP.html");

// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//     #[prop_or_default]
//     pub text: String,
// }

pub enum Msg {
    UpdateTermText(String),
    DisplayTooltip(String),
    HideTooltip,
}

pub struct Terminal {
    termtext: String,
    tooltiptext: String,
    tooltipvisible: bool,
}

impl Component for Terminal {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            termtext: "<h1 id=\"intro-text\">Click a tab above while you wait for content to load below!</h1>".to_string(),
            tooltiptext: "".to_string(),
            tooltipvisible: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTermText(html) => {
                self.termtext = html; // run the callback associated with onclick event
            }
            Msg::DisplayTooltip(text) => {
                self.tooltiptext = text;
                self.tooltipvisible = true;
            }
            Msg::HideTooltip => {
                self.tooltipvisible = false;
            }
        }
        true // return that the DOM element should be updated when an event is handled
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="intro">
                    <div class="tooltip" id="tooltip" style={ if self.tooltipvisible == true { "visibility: visible;" } else { "visibility: hidden;" } }><h4>{ self.tooltiptext.clone() }</h4></div>
                    <div class="term" style="animation: pop 4s ease 1;">
                        <div class="term-bar" id="term-bar" style="animation: pop 4s ease 1;">
                            <div class="term-tab" id="term-tab-0" style="animation: shrink-tab 7s ease 1;">
                                <TermBtn class="term-tab-btn" id="term-btn-0" onclick={ctx.link().callback(|_| Msg::UpdateTermText(COWSAY.to_string()))} onmouseover={ctx.link().callback(|_| Msg::DisplayTooltip("bokehnoroses@github:~$ echo 'Hello, World' | cowsay".to_string()))} onmouseleave={ctx.link().callback(|_| Msg::HideTooltip)} title="bokehnoroses@github:~$ echo 'Hello, World' | cowsay" />
                            </div>
                            <div class="term-tab" id="term-tab-1" style="animation: unhide-tab 8s ease 1;">
                                <TermBtn class="term-tab-btn" id="term-btn-1" onclick={ctx.link().callback(|_| Msg::UpdateTermText(P_INTRO.to_string()))} onmouseover={ctx.link().callback(|_| Msg::DisplayTooltip("bokehnoroses@github:~$ cat introduction.md | less".to_string()))} onmouseleave={ctx.link().callback(|_| Msg::HideTooltip) } title="bokehnoroses@github:~$ cat introduction.md | less" />
                            </div>
                            <div class="term-tab" id="term-tab-2" style="animation: unhide-tab 9s ease 1;">
                                <TermBtn class="term-tab-btn" id="term-btn-2" onclick={ctx.link().callback(|_| Msg::UpdateTermText(ACCOMP.to_string()))} onmouseover={ctx.link().callback(|_| Msg::DisplayTooltip("bokehnoroses@github:~$ cat accomplishments.md | less".to_string()))} onmouseleave={ctx.link().callback(|_| Msg::HideTooltip)} title="bokehnoroses@github:~$ cat accomplishments.md | less" />
                            </div>
                            <div class="term-tab" id="term-tab-3" style="animation: unhide-tab 10s ease 1;">
                                <TermBtn class="term-tab-btn" id="term-btn-3" onclick={ctx.link().callback(|_| Msg::UpdateTermText(U_SCRIPTS.to_string()))} onmouseover={ctx.link().callback(|_| Msg::DisplayTooltip("bokehnoroses@github:~$ cat useful_scripts.md | less".to_string()))} onmouseleave={ctx.link().callback(|_| Msg::HideTooltip)} title="bokehnoroses@github:~$ cat useful_scripts.md | less" />
                            </div>
                            <div class="term-tab" id="term-tab-4" style="animation: unhide-tab 11s ease 1;">
                                <TermBtn class="term-tab-btn" id="term-btn-4" onclick={ctx.link().callback(|_| Msg::UpdateTermText(WIP.to_string()))} onmouseover={ctx.link().callback(|_| Msg::DisplayTooltip("bokehnoroses@github:~$ cat wip.md | less".to_string()))} onmouseleave={ctx.link().callback(|_| Msg::HideTooltip)} title="bokehnoroses@github:~$ cat wip.md | less" />
                            </div>
                        </div>
                        <div class="term-body" id="term-body">
                            { Html::from_html_unchecked(AttrValue::from(self.termtext.clone())) }
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
