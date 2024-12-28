mod view;
mod update;
mod validation;

use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_console::log;

use crate::messages::journal_form::Msg;
use crate::api::journal::JournalApi;
use crate::models::journal::{Journal, Account};
use crate::models::journal_form::Model;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_submit: Callback<Journal>,
    pub on_cancel: Callback<()>,
    pub accounts: Vec<Account>,
    pub journal: Option<Journal>,
}

pub struct JournalForm {
    model: Model,
    api: JournalApi,
}

impl Component for JournalForm {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let component = Self {
            model: Model::default(),
            api: JournalApi::new(),
        };

        ctx.link().send_message(Msg::InitializeForm(ctx.props().journal.clone()));
        ctx.link().send_message(Msg::LoadAccounts);

        component
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.view(ctx)
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.update(ctx, msg)
    }
}