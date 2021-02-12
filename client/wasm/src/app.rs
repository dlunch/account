use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::router::Router;

use super::components;
use super::route::AppRoute;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
           <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Register => html!{<components::Register />},
                        AppRoute::Login => html!{<components::Login />},
                        AppRoute::Index => html!{<components::Index/>},
                    }
                })
            />
        }
    }
}
