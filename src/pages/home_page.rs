use leptos::prelude::*;
use crate::components::{Header, DashboardHeader, DashboardChart};

#[component]
pub fn HomePage() -> impl IntoView {


        view! {
            <body class="bg-gray-900 overflow-x-hide">
                <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">
                <Header />
                <DashboardHeader />
                
            </div>
        </body>
    }
}