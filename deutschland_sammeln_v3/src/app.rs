use std::ops::Deref;
use std::sync::Arc;
use chrono::{NaiveDateTime};
use leptos::html;
use leptos::prelude::*;
use leptos::web_sys::SubmitEvent;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/deutschland_sammeln_v3.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        "Geben sie einen Gemeindenamen ein"
        <GemeindeNameInput/>
        <br/>
        <GemeindeListe/>
        <br/>
    }
}

#[component]
fn GemeindeNameInput(

) -> impl IntoView {
    let (name, set_name) = signal("".to_string());

    let input_element: NodeRef<html::Input> = NodeRef::new();
    
    let clicked: RwSignal<bool> = RwSignal::new(false);

    let database_deferer = Action::new(|input: &Arc<str>|  {
        let input = input.to_owned();
        async move {defer_to_database_entry_handler(input).await.unwrap()}
    });
    
    let database_duplicate_result = database_deferer.value();
    
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();
        
        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();

        let value_arc: Arc<str> = Arc::from(value.deref());

        set_name.set(value);
        clicked.set(true);
        
        
        database_deferer.dispatch(value_arc.clone());
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <Show
            when=move || {clicked.get()}
            fallback=|| view! {}
        >
                <Show
                    when=move || {database_duplicate_result.get().unwrap_or(false)}
                    fallback=|| view! { <p> "Die Gemeinde wurde der Liste hinzugefügt" </p> }
                >
                    <p> "Diese Gemeinde ist bereits in der Liste vorhanden" </p>
                </Show>
        </Show>
    }
}

#[cfg(feature = "ssr")]
use crate::database::enter_name_into_database;
#[cfg(feature = "ssr")]
use crate::database::get_unique_entries_from_database;

#[server]
async fn defer_to_database_entry_handler(name: Arc<str>) -> Result<bool, ServerFnError> {
    enter_name_into_database(name).await
}

#[component]
fn GemeindeListe() -> impl IntoView {
    view! {
        
    }
} 

#[server]
async fn get_entries_from_database(start_time: Option<NaiveDateTime>) -> Result<Vec<String>, ServerFnError> {
    get_unique_entries_from_database(start_time).await
}