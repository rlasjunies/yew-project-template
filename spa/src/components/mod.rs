mod app_container;
mod page1;
mod page2;

use yew_router::Switch;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/app/main"]
    Main,
    #[to = "/app/page1"]
    Page1,
    #[to = "/app/page2"]
    Page2,
    // #[to = "/app/{pagename}"]
    // Page(String),
}

pub use app_container::AppContainer;
pub use page1::Page1;
pub use page2::Page2;
