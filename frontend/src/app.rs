use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::journal_list::JournalList;
use crate::components::journal_form::JournalForm;
use crate::route::Route;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <JournalList /> },
        Route::NewJournal => html! {
            <JournalForm
                accounts={vec![]} // TODO:
                on_submit={Callback::from(|_| {})} // TODO:
                on_cancel={Callback::from(|_| {})} // TODO:
                journal={None}
            />
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="container">
                <header>
                    <h1>{"会計システム"}</h1>
                    <nav>
                        <Link<Route> to={Route::Home}>{"仕訳一覧"}</Link<Route>>
                        <Link<Route> to={Route::NewJournal}>{"仕訳登録"}</Link<Route>>
                    </nav>
                </header>
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}