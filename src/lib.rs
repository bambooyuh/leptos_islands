use leptos::prelude::*;

mod components;
mod errors;
mod models;
mod pages;

use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet};
use leptos_router::{
    components::{FlatRoutes, Route, Router}, StaticSegment,
};
use pages::{HomePage, TeamPage};
use crate::components::Header;
#[cfg(feature = "ssr")]
pub mod fallback;
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true/>
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
    provide_meta_context();


    let script_url = "https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js";
    let script_gl_url = "https://cdn.jsdelivr.net/npm/echarts-gl@2.0.9/dist/echarts-gl.min.js";

    view! {
        <Stylesheet id="leptos" href="/pkg/flash1.css"/>
        <Meta name="description" content="Dashboard app."/>

        <script src=script_url></script>
        <script src=script_gl_url></script>
        <Router>
            <main>
                <FlatRoutes fallback=|| "Not found.">
                <Route path=StaticSegment("/") view=move || {

                    view! {
                        <HomePage />
                    }
                }/>
                <Route path=StaticSegment("/team") view=move || {
                    view! {
                        <TeamPage />
                    }
                }/>
                </FlatRoutes>
            </main>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
