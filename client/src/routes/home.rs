use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="banner">
                <div>
                    <h1>
                        { "ryan christoffersen" }
                    </h1>
                    <p>
                        <i>
                            { "what is the role of technology in regeneration?" }
                        </i>
                    </p>
                    <p>
                        { "researcher, developer, storyteller" }
                    </p>
                </div>
            </div>
        }
    }
}
