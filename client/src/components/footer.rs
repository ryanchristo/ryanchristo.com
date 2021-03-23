use yew::prelude::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <footer class="footer">
                <ul>
                    <li>
                        <a
                            href="https://twitter.com/ryanchrypto"
                            rel="noopener noreferrer"
                            target="_blank"
                        >
                            { "twitter" }
                        </a>
                    </li>
                    <li>
                        <a
                            href="https://github.com/ryanchrypto"
                            rel="noopener noreferrer"
                            target="_blank"
                        >
                            { "github" }
                        </a>
                    </li>
                    // <li>
                    //     <a
                    //         href="https://www.youtube.com/channel/UC-C3V5kim-kkXlrkQ_VT8BQ"
                    //         rel="noopener noreferrer"
                    //         target="_blank"
                    //     >
                    //         { "youtube" }
                    //     </a>
                    // </li>
                    // <li>
                    //     <a
                    //         href="https://www.patreon.com/ryanchristo"
                    //         rel="noopener noreferrer"
                    //         target="_blank"
                    //     >
                    //         { "patreon" }
                    //     </a>
                    // </li>
                </ul>
            </footer>
        }
    }
}