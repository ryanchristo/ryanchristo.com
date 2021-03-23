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
        // let mut section1: Vec<Link> = Vec::new();
        // let mut section2: Vec<Link> = Vec::new();
        // let mut section3: Vec<Link> = Vec::new();

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

        // ecology

        // section1.push(Link  {
        //     name: "untitled".to_string(),
        //     root: "blog".to_string(),
        //     slug: "untitled".to_string(),
        // });

        // menu.push(Section {
        //     title: "ecology".to_string(),
        //     links: section1,
        // });

        // technology

        // section2.push(Link  {
        //     name: "untitled".to_string(),
        //     root: "blog".to_string(),
        //     slug: "untitled".to_string(),
        // });

        // menu.push(Section {
        //     title: "technology".to_string(),
        //     links: section2,
        // });

        // journey

        // section3.push(Link  {
        //     name: "untitled".to_string(),
        //     root: "blog".to_string(),
        //     slug: "untitled".to_string(),
        // });

        // menu.push(Section {
        //     title: "journey".to_string(),
        //     links: section3,
        // });

        // index
        let path = include_str!("../../../docs/blog/index.md");

        // // index
        // let mut path = include_str!("../../../docs/blog/index.md");

        // // blog
        // if self.props.slug == "a-degenerate-regenerate" {
        //     path = include_str!("../../../docs/blog/a-degenerate-regenerate.md")
        // } else if self.props.slug == "entering-the-cosmos" {
        //     path = include_str!("../../../docs/blog/entering-the-cosmos.md")
        // } else if self.props.slug == "why-regeneration-now" {
        //     path = include_str!("../../../docs/blog/why-regeneration-now.md")
        // }

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
