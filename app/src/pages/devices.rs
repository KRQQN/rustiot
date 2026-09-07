use crate::components::{devices::main::Devices, side_menu::SideMenu};
use leptos::prelude::*;

#[component]
pub fn DevicesPage() -> impl IntoView {
    view! {
        <div class="flex min-h-screen w-full bg-slate-950">
            <SideMenu />

            <main class="min-w-0 flex-1">
                <Devices />
            </main>
        </div>
    }
}
