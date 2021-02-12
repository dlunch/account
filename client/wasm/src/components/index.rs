use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use crate::router::{AppAnchor, AppRoute};

pub struct Index {}

pub enum Msg {}

impl Component for Index {
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
            <div class="index">
                <AppAnchor route=AppRoute::Login> { "Login" } </AppAnchor>
                <AppAnchor route=AppRoute::Register> { "Register" } </AppAnchor>
            </div>
        }
    }
}
