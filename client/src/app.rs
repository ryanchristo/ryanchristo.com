use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    footer::Footer,
    header::Header,
};

use crate::routes::{
    AppRoute,
    about::About,
    blog::Blog,
    home::Home,
    film::Film,
    tech::Tech,
};

pub struct App {
    current_route: Option<AppRoute>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    Route(Route),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let route = route_service.get_route();
        App {
            current_route: AppRoute::switch(route),
            router_agent,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(route) => {
                self.current_route = AppRoute::switch(route);
                true
            }
        }
    }

    fn view(&self) -> Html {
        if let Some(route) = &self.current_route {
            match route {
                // home
                AppRoute::Home => html! {
                    <main class="theme">
                        <div class="container">
                            <div class="background" />
                            <Header />
                            <Home />
                            <Footer />
                        </div>
                    </main>
                },
                // about
                AppRoute::About => html! {
                    <main class="theme">
                        <div class="container">
                            <Header />
                            <About />
                            <Footer />
                        </div>
                    </main>
                },
                // tech
                AppRoute::Tech => html! {
                    <main class="theme">
                        <div class="container">
                            <Header />
                            <Tech />
                            <Footer />
                        </div>
                    </main>
                },
                // film
                AppRoute::Film => html! {
                    <main class="theme">
                        <div class="container">
                            <Header />
                            <Film />
                            <Footer />
                        </div>
                    </main>
                },
                // blog
                AppRoute::Blog => html! {
                    <main class="theme">
                        <div class="container">
                            <Header />
                            <Blog slug="" />
                            <Footer />
                        </div>
                    </main>
                },
                // blog page
                AppRoute::BlogPage(slug) => html! {
                    <main class="theme">
                        <div class="container">
                            <Header />
                            <Blog slug=slug />
                            <Footer />
                        </div>
                    </main>
                },
            }
        } else {
            html! { "route not found" }
        }
    }
}
