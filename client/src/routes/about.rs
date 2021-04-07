use yew::prelude::*;

use crate::utils::markdown;

pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        About { }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {

        // markdown
        let intro = include_str!("../../../content/about/about-intro.md");
        let timeline = include_str!("../../../content/about/about-timeline.md");

        html! {
            <div class="container">
                <div class="content">
                    <div class="content-item">
                        { markdown::render_markdown(intro) }
                    </div>
                    <div class="content-item" style="text-align:center">
                        { markdown::render_markdown(timeline) }
                    </div>
                </div>
            </div>
        }
    }
}

