use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use crate::assets::shared::sidebar::Sidebar;
use crate::assets::shared::header::Header;
use crate::assets::pages::home::Home;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    let css=r"
        html, body {
            margin: 0;
            height: 100%;
        }
        body {
            font-family: sans-serif;
            text-align: center;
        }";
    view! {
        <!DOCTYPE html>
        <html lang="ja-JP">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <style inner_html=css />
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
    let css=r"
        div.content {
            display: flex;
            flex-direction:row;
            height: calc(100% - 100px); /* 画面全体からヘッダー分を引く */
        }

        .main {
            flex: 1; /* 余った領域をすべて埋める */
            background-color: lightgray;
            overflow-y: auto;
        }";
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_test.css"/>
        // sets the document title
        <Title text="Welcome to Leptos"/>
        
        // content for this welcome page
        <Header/>
        <style inner_html=css />
        <div class="content">
            <Sidebar/>
            <main class="main">
                <Router>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=Home/>
                    </Routes>
                </Router>
            </main>
        </div>
        
    }
}
