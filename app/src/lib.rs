mod components;
mod data;
mod e_charts;
mod pages;
pub mod server_fns;
mod structs;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    SsrMode, StaticSegment,
};

pub use components::hero::Hero;
pub use pages::{dashboard::DashboardArea, devices::DevicesPage, login_home::LoggedInHomePage};
pub use structs::app_state::AppState;

use crate::components::{challenges::Problems, platform::Platform, solutions::Solutions};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script src="https://cdn.tailwindcss.com"></script>
                <script src="https://cdn.jsdelivr.net/npm/echarts@5/dist/echarts.min.js"></script>
                <script src="/assets/charts.js"></script>
                <AutoReload options=options.clone()/>
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
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rustiot.css"/>

        <Title text="NEXTIOT"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route
                        path=StaticSegment("/login")
                        view=LoggedInHomePage
                        ssr=SsrMode::Async
                        />
                    <Route
                        path=StaticSegment("/dashboard")
                        view=DashboardArea
                        ssr=SsrMode::Async
                        />
                    <Route
                        path=StaticSegment("/devices")
                        view=DevicesPage
                        ssr=SsrMode::Async
                        />
                    <Route
                        path=StaticSegment("")
                        view=HomePage
                        ssr=SsrMode::Async
                        />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div id="hero" class="">
                <Hero/>
        </div>
        <Problems/>
        <Solutions/>
        <Platform/>
    }
}
