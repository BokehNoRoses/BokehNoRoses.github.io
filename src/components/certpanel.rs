use yew::prelude::*;

pub enum Msg {
    CertsView,
}

pub struct CertPanel {
    certsview: bool,
}

impl Component for CertPanel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { certsview: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CertsView => {
                if self.certsview == true {
                } else {
                    self.certsview = true;
                }
            }
        }
        true // return that the DOM should be updated when event is handled
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="certs" onmouseenter={ctx.link().callback(|_| Msg::CertsView)}>
                    <div class="certs-card" id="card-1" style={ if self.certsview == true { "display: flex; animation: jump 1.7s linear 1 forwards;" } else { "display: none;" } }>
                        <img src="assets/images/CompTIA_Security+.png" alt="CompTIA Security+"/>
                        <h2 class="text">{ "CompTIA Security+ SY0-601" }</h2>
                        <h4 class="text">{ "Obtained Date: November 2021" }</h4>
                        <h4 class="text">{ "Expiration Date: November 2027" }</h4>
                    </div>
                    <div class="certs-card" id="card-2" style={ if self.certsview == true { "display: flex; animation: jump 2.5s linear 1 forwards;" } else { "display: none;" } }>
                        <img src="assets/images/AZ-104.png" alt="Azure System Administrator Associate"/>
                        <h2 class="text">{ "Azure Administrator AZ-104" }</h2>
                        <h4 class="text">{ "Obtained Date: November 2022" }</h4>
                        <h4 class="text">{ "Expiration Date: November 2025" }</h4>
                    </div>
                    <div class="certs-card" id="card-3" style={ if self.certsview == true { "display: flex; animation: jump 3.3s linear 1 forwards;" } else { "display: none;" } }>
                        <img src="assets/images/CompTIA_Linux+.png" alt="CompTIA Linux+"/>
                        <h2 class="text">{ "CompTIA Linux+ XK0-005" }</h2>
                        <h4 class="text">{ "Obtained Date: March 2024" }</h4>
                        <h4 class="text">{ "Expiration Date: November 2027" }</h4>
                    </div>
                </div>
            </>
        }
    }
}
