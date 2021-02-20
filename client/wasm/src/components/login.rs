use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

use crate::context::Context;
use crate::grpc::{AuthPromiseClient, LoginRequest};
use crate::router::{change_route, AppRoute};

pub struct Login {
    link: ComponentLink<Self>,
    username: NodeRef,
    password: NodeRef,
}

pub enum Msg {
    Submit,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            username: NodeRef::default(),
            password: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit => {
                let username = self.username.cast::<HtmlInputElement>().unwrap().value();
                let password = self.password.cast::<HtmlInputElement>().unwrap().value();

                let login_request = LoginRequest::new();
                login_request.setUsername(&username);
                login_request.setPassword(&password);

                let context = Context::get();
                let grpc_host = context.grpc_host.clone();
                spawn_local(async move {
                    let auth_client = AuthPromiseClient::new(&grpc_host, JsValue::NULL, JsValue::NULL);
                    auth_client.login(JsValue::from(login_request), JsValue::NULL).await;

                    change_route(AppRoute::Index);
                });
            }
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="login">
                <form>
                    <p> <label for="username">{ "Username :" }</label> <input id="username" type="text" ref=self.username.clone() /> </p>
                    <p> <label for="password">{ "Password :" }</label> <input id="password" type="password" ref=self.password.clone() /> </p>
                    <p> <button onclick=self.link.callback(|_| Msg::Submit) type="button"> { "Submit" }</button> </p>
                </form>
            </div>
        }
    }
}
