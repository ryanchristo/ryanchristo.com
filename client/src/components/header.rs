use yew::prelude::*;
// use yew_router::prelude::*;

// use crate::routes::AppRoute;

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="header">
                <ul>
                    <li>
                        <a href="/" alt="home">
                            { "home" }
                        </a>
                    </li>
                    <li>
                        <a href="/about" alt="about">
                            { "about" }
                        </a>
                    </li>
                    <li>
                        <a href="/tech" alt="tech">
                            { "tech" }
                        </a>
                    </li>
                    <li>
                        <a href="/film" alt="film">
                            { "film" }
                        </a>
                    </li>
                    // <li>
                    //     <a href="/blog" alt="blog">
                    //         { "blog" }
                    //     </a>
                    // </li>
                    // TODO: fix scroll to top on route change
                    // <RouterAnchor<AppRoute> route=AppRoute::Home>
                    //     { "home" }
                    // </RouterAnchor<AppRoute>>
                    // <RouterAnchor<AppRoute> route=AppRoute::About>
                    //     { "about" }
                    // </RouterAnchor<AppRoute>>
                    // <RouterAnchor<AppRoute> route=AppRoute::Tech>
                    //     { "tech" }
                    // </RouterAnchor<AppRoute>>
                    // <RouterAnchor<AppRoute> route=AppRoute::Film>
                    //     { "film" }
                    // </RouterAnchor<AppRoute>>
                    // <RouterAnchor<AppRoute> route=AppRoute::Blog>
                    //     { "blog" }
                    // </RouterAnchor<AppRoute>>
                </ul>
            </nav>
        }
    }
}