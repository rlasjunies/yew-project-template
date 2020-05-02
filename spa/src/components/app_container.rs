use super::AppRoute;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{service::RouteService, Switch};

use super::{Page1, Page2};

pub struct AppContainer {
    route_service: RouteService<()>,
}

impl Component for AppContainer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            route_service: RouteService::new(),
        }
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
                <menu>
                    <a href="/app/main">{ "Home" }</a>
                    <a href="/app/page1">{ "Page1" }</a>
                    <a href="/app/page2">{ "Page2" }</a>
                </menu>
                <div class="main-pane full-height">
                    { self.view_app_content() }
                </div>
            </div>
        }
    }
}

impl AppContainer {
    fn view_app_content(&self) -> Html {
        let route = self.route_service.get_route();
        match AppRoute::switch(route) {
            Some(AppRoute::Main) => html! {
                <>
                    <h1>{"Welcome in this yew template project"}</h1>
                </>
            },
            Some(AppRoute::Page1) => html!{
                <>
                    <Page1/>
                </>
            },
            Some(AppRoute::Page2) => html!{
                <>
                    <Page2/>
                </>
            },
            // Some(AppRoute::Page(page)) => html! {
            //     <>
            //     // <Page1 />
            //     </>
            // },
            _ => html! {
                <div>{ "Route not found" }</div>
            },
        }
    }
}
