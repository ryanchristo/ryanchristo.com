use yew::prelude::*;

use crate::utils::markdown;

pub struct Film;

impl Component for Film {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Film {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {

        // markdown
        let intro = include_str!("../../../docs/film/film-intro.md");
        let projects = include_str!("../../../docs/film/film-projects.md");

        html! {
            <div class="container">
                <div class="content">
                    <div class="content-item">
                        { markdown::render_markdown(intro) }
                    </div>
                    <div class="content-item" style="text-align:center">
                        { markdown::render_markdown(projects) }
                    </div>
                </div>
            </div>
        }
    }
}

