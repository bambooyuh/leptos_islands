use leptos::prelude::*;
use crate::{
    components::{ Toast, ToastMessage, Header},
};


#[island]
pub fn Island() -> impl IntoView {

    const ADD_BUTTON_STYLE: &str = "bg-[#7734e7] px-8 py-2 rounded text-white
    transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

    let (if_show_modal, set_if_show_modal) = signal(false);

    // for showing/animating the toast message
    let (if_show_toast, set_if_show_toast) = signal(false);
    let (toast_message, set_toast_message) = signal(ToastMessage::new());

    // Retrieves data one time on page load with retry logic


        let on_click = move |_| {
            set_if_show_modal(!if_show_modal());
        };


    view! {
        <body class="bg-gray-900 overflow-x-hide">
            <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">

            <Header />
            <Toast
                toast_message
                if_appear=if_show_toast
                set_if_appear=set_if_show_toast
            />

                <div class="mt-20">

                    <div class="text-white flex flex-col w-full mx-auto items-center justify">

                    

                        <div class="flex flex-row w-full max-w-[52rem]">

                            <div class="pr-4 mt-4 text-xl">"Members"</div>
                            <hr class="w-full max-w-[48rem] pl-4 pr-4 pt-4 mt-8 mr-4" />

                            <button on:click=on_click class=ADD_BUTTON_STYLE>
                            "Add"
                            </button>


                        </div>

                    
                    </div>
                </div>                     
            </div>
        </body>
    }
}



#[component]
pub fn TeamPage() -> impl IntoView {

    view! {
        <Island />
    }
    
}



/*
NOTES:

Why is map() chained to  move || { get_persons_info.get().map(|data| {
get_persons_info.get() returns an Option<Result<Vec<Person>, Error>>. 
You need to handle the fact that it might be None (i.e., data not yet available or an error).
.map() is used to transform the Option if it contains a Some value.
If get_persons_info.get() is Some(Result), the closure inside .map() gets executed with the Result<Vec<Person>, Error> as its argument.

The code below would work...
let result = get_persons_info.get();
match result {
    Some(data) => { /* Handle the Result<Vec<Person>, Error> here */ }
    None => { /* Handle the case when data is not yet available */ }
}

However....map() is a more concise way of dealing with the Some case, as it lets you skip the None handling and focus on processing the data.


*/