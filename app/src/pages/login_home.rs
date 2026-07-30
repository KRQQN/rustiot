use leptos::prelude::*;

use crate::components::signup_form::RegisterForm;

#[component]
pub fn LoggedInHomePage() -> impl IntoView {
    view! {
        <div class="w-[80%] bg-blue h-full">
            <RegisterForm/>
        </div>
    }
}
