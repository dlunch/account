use yew_router::{
    agent::{RouteAgentDispatcher, RouteRequest},
    components::RouterAnchor,
    route::Route,
    Switch,
};

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

pub fn change_route(route: AppRoute) {
    let mut router = RouteAgentDispatcher::<()>::new();
    router.send(RouteRequest::ChangeRoute(Route::from(route)));
}
