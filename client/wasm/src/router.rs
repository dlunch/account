use yew_router::{components::RouterAnchor, Switch};

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/register"]
    Register,
    #[to = "/login"]
    Login,
    #[to = "/"]
    Index,
}

pub type AppAnchor = RouterAnchor<AppRoute>;
