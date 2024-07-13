use yew::prelude::*;

// https://github.com/yewstack/yew/blob/master/examples/inner_html
const PROJ0: &str = include_str!("../../assets/excerpts/PROJ0.html");
const PROJ1: &str = include_str!("../../assets/excerpts/PROJ1.html");
const PROJ2: &str = include_str!("../../assets/excerpts/PROJ2.html");
const PROJ3: &str = include_str!("../../assets/excerpts/PROJ3.html");

// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//     #[prop_or_default]
//     pub title: String,
//     pub class: String,
//     pub id: String,
//     pub onsignal: Callback<()>,
// }

pub enum Msg {
    NextCarousel,
    PrevCarousel,
    SelectCarousel(u8),
}

pub struct Carousel {
    ctext: String,
    counter: u8,
}

impl Component for Carousel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ctext: PROJ0.to_string(),
            counter: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NextCarousel => {
                let count: u8 = self.counter.clone();
                self.counter = (count + 1) % 4;
                self.ctext = match self.counter {
                    0 => PROJ0.to_string(),
                    1 => PROJ1.to_string(),
                    2 => PROJ2.to_string(),
                    3 => PROJ3.to_string(),
                    _ => PROJ0.to_string(),
                }
            }
            Msg::PrevCarousel => {
                let count: u8 = self.counter.clone();
                self.counter = (count + 3) % 4;
                self.ctext = match self.counter {
                    0 => PROJ0.to_string(),
                    1 => PROJ1.to_string(),
                    2 => PROJ2.to_string(),
                    3 => PROJ3.to_string(),
                    _ => PROJ0.to_string(),
                }
            }
            Msg::SelectCarousel(num) => {
                self.counter = num;
                self.ctext = match self.counter {
                    0 => PROJ0.to_string(),
                    1 => PROJ1.to_string(),
                    2 => PROJ2.to_string(),
                    3 => PROJ3.to_string(),
                    _ => PROJ0.to_string(),
                }
            }
        }
        true // return that the DOM element should not updated and view should not be called again
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="projects">
                <div class="term">
                    <div class="term-bar"><h2 id="gallery-text">{ "Gallery of Works" }</h2></div>
                    <div class="term-body" style="display: flex; justify-content: center; align-items: center; padding: 0 0 !important; margin: 0 0 !important; height: 93% !important; width: 100% !important;">
                        <div class="carousel-sidebar">
                            <button class="carousel-btn" id="c-bk" onclick={ctx.link().callback(|_| Msg::PrevCarousel)}>{ "<" }</button>
                        </div>
                        <div class="carousel-body">
                            <div class="carousel-picture">
                                { Html::from_html_unchecked(AttrValue::from(self.ctext.clone())) }
                            </div>
                            <div class="carousel-btn-bar">
                                <input type="radio" id="c-btn-0" name="carousel-input" checked={ if self.counter == 0 { true } else { false } } onclick={ctx.link().callback(|_| Msg::SelectCarousel(0))}/>
                                <input type="radio" id="c-btn-1" name="carousel-input" checked={ if self.counter == 1 { true } else { false } } onclick={ctx.link().callback(|_| Msg::SelectCarousel(1))}/>
                                <input type="radio" id="c-btn-2" name="carousel-input" checked={ if self.counter == 2 { true } else { false } } onclick={ctx.link().callback(|_| Msg::SelectCarousel(2))}/>
                                <input type="radio" id="c-btn-3" name="carousel-input" checked={ if self.counter == 3 { true } else { false } } onclick={ctx.link().callback(|_| Msg::SelectCarousel(3))}/>
                            </div>
                        </div>
                        <div class="carousel-sidebar">
                            <button class="carousel-btn" id="c-fw" onclick={ctx.link().callback(|_| Msg::NextCarousel)}>{ ">" }</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
