use yew::prelude::*;

use crate::components::sidebar::{
    Link,
    Section,
    Sidebar,
};

use crate::utils::markdown;

pub struct Blog {
    pub props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub slug: String,
}

impl Component for Blog {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Blog { props }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let mut menu: Vec<Section> = Vec::new();
        
        let mut section0: Vec<Link> = Vec::new();
        let mut section1: Vec<Link> = Vec::new();

        // blog

        section0.push(Link  {
            name: "index".to_string(),
            root: "blog".to_string(),
            slug: "".to_string(),
        });

        menu.push(Section {
            title: "blog".to_string(),
            links: section0,
        });

        // updates

        section1.push(Link  {
            name: "Beginning Again".to_string(),
            root: "blog".to_string(),
            slug: "beginning-again".to_string(),
        });

        menu.push(Section {
            title: "updates".to_string(),
            links: section1,
        });

        // index
        let mut path = include_str!("../../../content/blog/index.md");

        // posts
        if self.props.slug == "beginning-again" {
            path = include_str!("../../../content/blog/000000_beginning_again.md")
        }

        html! {
            <div class="container">
                <Sidebar menu=menu />
                <div class="content">
                    <div class="content-item">
                        { markdown::render_markdown(path) }
                    </div>
                </div>
            </div>
        }
    }
}
