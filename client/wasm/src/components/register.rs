use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement};
use yew::prelude::{html, html::NodeRef, Component, ComponentLink, Html, ShouldRender};

use crate::context::Context;
use crate::grpc::{AuthPromiseClient, RegisterRequest};

pub struct Register {
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
        Self {
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

                let context = Context::get();
                let grpc_host = context.grpc_host.clone();
                spawn_local(async move {
                    let auth_client = AuthPromiseClient::new(&grpc_host, JsValue::NULL, JsValue::NULL);
                    auth_client.register(JsValue::from(register_request), JsValue::NULL).await;
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
            <div class="register">
                <form>
                    <p> <label for="username">{ "Username :" }</label> <input id="username" type="text" ref=self.username.clone() /> </p>
                    <p> <label for="password">{ "Password :" }</label> <input id="password" type="password" ref=self.password.clone() /> </p>
                    <p> <label for="password_again">{ "Password Again :" }</label> <input id="password_again" type="password" ref=self.password_again.clone() /> </p>
                    <p> <button onclick=self.link.callback(|_| Msg::Submit) type="button"> { "Submit" }</button> </p>
                </form>
            </div>
        }
    }
}
