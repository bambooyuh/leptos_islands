use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_url;


const INPUT_STYLE: &str = "border-b-0 border-[#7734e7] h-8 text-white ml-4 mr-4
    hover:border-b-2";
const INPUT_STYLE_SELECTED: &str = "border-b-2 border-[#9734e7] h-8 text-white ml-4
    mr-4 hover:border-b-2";

#[component]
pub fn Header() -> impl IntoView {
    // Create a signal to track the current path
    let (current_path, set_current_path) = signal(String::new());

    // Use use_url() to get the current URL
    let url_signal = use_url();

    // Create an effect to update the current path whenever the URL changes
    Effect::new(move |_| {
        let url = url_signal.get();
        let path = url.path();
        set_current_path(path.to_string());
    });

    view! {
        <div class="flex mx-auto align-center items-center w-full h-12 pt-8 px-20 bg-gray-900 top-0 fixed">
            <nav class="flex flex-row w-full max-w-[52rem] h-12">
                <div class={move || get_style_from_url(&current_path, "/")}>
                    <A href="/">"Dashboard"</A>
                </div>
                <div class={move || get_style_from_url(&current_path, "/team")}>
                    <A href="/team">"Team"</A>
                </div>
            </nav>
        </div>
    }
}

fn get_style_from_url<'a, 'b>(url: &'a ReadSignal<String>, match_url: &'a str) -> &'b str {
    if url() == match_url {
        INPUT_STYLE_SELECTED
    } else {
        INPUT_STYLE
    }
}