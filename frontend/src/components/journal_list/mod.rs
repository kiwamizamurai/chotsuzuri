mod update;
mod view;
mod helpers;

pub use helpers::format_currency;

use crate::api::journal::JournalApi;
use crate::messages::journal_list::Msg;
use crate::models::journal_list::Model;

use yew::prelude::*;

pub struct JournalList {
    model: Model,
    api: JournalApi,
}

impl Component for JournalList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let component = Self {
            model: Model::new(),
            api: JournalApi::new(),
        };

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