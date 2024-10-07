use leptos::prelude::*;

#[component]
pub fn DashboardWidget(title: String, value: String) -> impl IntoView {
    view! {
        <div class="flex flex-col h-36 w-full max-w-[21rem] bg-[#283653] rounded
            px-10 py-4 justify-center">

            <div class="text-white text-4xl">{value}</div>
            <div class="text-stone-400">{title}</div>
        </div>
    }
}