use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Page1 {}

impl Component for Page1 {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // self.props = props;
        // true // This will always re-render when new props are provided.
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="full-height">
                <h1>{"Page1"}</h1>
            </div>
        }
    }
}
