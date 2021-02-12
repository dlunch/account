use wasm_bindgen::JsValue;
use web_sys::{window, HtmlInputElement};
use yew::prelude::{html, html::NodeRef, Component, ComponentLink, Html, ShouldRender};

use crate::context::Context;
use crate::grpc::{AuthClient, RegisterRequest};

pub struct Register {
    auth_client: AuthClient,
    link: ComponentLink<Self>,
    username: NodeRef,
    password: NodeRef,
    password_again: NodeRef,
}

pub enum Msg {
    Submit,
}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let context = Context::get();
        let auth_client = AuthClient::new(&context.grpc_host, JsValue::NULL, JsValue::NULL);

        Self {
            auth_client,
            link,
            username: NodeRef::default(),
            password: NodeRef::default(),
            password_again: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit => {
                let username = self.username.cast::<HtmlInputElement>().unwrap().value();
                let password = self.password.cast::<HtmlInputElement>().unwrap().value();
                let password_again = self.password_again.cast::<HtmlInputElement>().unwrap().value();

                if password != password_again {
                    window().unwrap().alert_with_message("Password and password check doesn't match").unwrap();

                    return false;
                }

                let register_request = RegisterRequest::new();
                register_request.setUsername(&username);
                register_request.setPassword(&password);

                self.auth_client
                    .register(JsValue::from(register_request), JsValue::NULL, JsValue::UNDEFINED);
            }
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="register">
                <p>
                    <label for="username">{ "Username :" }</label> <input id="username" type="text" ref=self.username.clone() />
                    <label for="password">{ "Password :" }</label> <input id="password" type="password" ref=self.password.clone() />
                    <label for="password_again">{ "Password Again :" }</label> <input id="password_again" type="password" ref=self.password_again.clone() />
                    <button onclick=self.link.callback(|_| Msg::Submit)> { "Submit" }</button>
                </p>
            </div>
        }
    }
}
