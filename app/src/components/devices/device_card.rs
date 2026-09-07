use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// temporary
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Device {
    pub id: i64,
    pub name: String,
    pub device_type: String,
    pub status: String,
    pub last_seen: Option<String>,
}

#[component]
pub fn DeviceCard(device: Device) -> impl IntoView {
    let is_online = device.status == "online";

    let status_dot = if is_online {
        "h-2 w-2 rounded-full bg-green-400"
    } else {
        "h-2 w-2 rounded-full bg-slate-600"
    };

    let status_label = if is_online { "Online" } else { "Offline" };

    let status_label_class = if is_online {
        "text-green-400"
    } else {
        "text-slate-500"
    };

    let last_seen = device.last_seen.unwrap_or_else(|| "Never".to_string());

    let icon = device_icon(&device.device_type);

    view! {
        <article class="
            group
            relative
            overflow-hidden
            rounded-2xl
            border
            border-slate-800
            bg-slate-900/60
            p-5
            shadow-lg
            transition
            duration-200
            hover:-translate-y-1
            hover:border-slate-700
            hover:bg-slate-900
        ">
            <div class="
                pointer-events-none
                absolute
                -right-12
                -top-12
                h-32
                w-32
                rounded-full
                bg-blue-500/10
                blur-3xl
            "></div>

            <div class="relative">

                <div class="
                    flex
                    items-start
                    justify-between
                ">
                    <div class="
                        flex
                        h-11
                        w-11
                        items-center
                        justify-center
                        rounded-xl
                        border
                        border-blue-400/20
                        bg-blue-500/10
                        text-xl
                        text-blue-300
                    ">
                        {icon}
                    </div>

                    <div class="
                        flex
                        items-center
                        gap-2
                        text-xs
                        font-medium
                    ">
                        <span class=status_dot></span>

                        <span class=status_label_class>
                            {status_label}
                        </span>
                    </div>
                </div>

                <div class="mt-6">
                    <h2 class="
                        truncate
                        text-base
                        font-semibold
                        text-slate-100
                    ">
                        {device.name}
                    </h2>

                    <p class="
                        mt-1
                        text-xs
                        text-slate-500
                    ">
                        {device.device_type}
                    </p>
                </div>

                <div class="
                    mt-6
                    flex
                    items-center
                    justify-between
                    border-t
                    border-slate-800
                    pt-4
                ">
                    <span class="
                        text-xs
                        text-slate-600
                    ">
                        "Last seen"
                    </span>

                    <span class="
                        text-xs
                        font-medium
                        text-slate-500
                    ">
                        {last_seen}
                    </span>
                </div>
            </div>
        </article>
    }
}

#[component]
pub fn EmptyDevices(on_add: Callback<()>) -> impl IntoView {
    view! {
        <div class="
            flex
            min-h-[320px]
            flex-col
            items-center
            justify-center
            rounded-2xl
            border
            border-slate-800
            bg-slate-900/40
            px-6
            text-center
        ">
            <div class="
                mb-5
                flex
                h-14
                w-14
                items-center
                justify-center
                rounded-2xl
                border
                border-slate-800
                bg-slate-900
                text-2xl
                text-slate-600
            ">
                "⌁"
            </div>

            <h2 class="
                text-base
                font-semibold
                text-slate-300
            ">
                "No devices yet"
            </h2>

            <p class="
                mt-2
                text-sm
                text-slate-500
            ">
                "Add your first device to get started."
            </p>

            <button
                type="button"
                class="
                    mt-5
                    rounded-xl
                    border
                    border-blue-400/20
                    bg-blue-500/10
                    px-4
                    py-2.5
                    text-sm
                    font-medium
                    text-blue-300
                    transition
                    hover:border-blue-400/30
                    hover:bg-blue-500/20
                "
                on:click=move |_| on_add.run(())
            >
                "+ Add device"
            </button>
        </div>
    }
}

fn device_icon(device_type: &str) -> &'static str {
    match device_type {
        "Laptop" => "⌨",
        "Desktop" => "▣",
        "Mobile" => "▯",
        "Tablet" => "▤",
        "Server" => "◈",
        _ => "◇",
    }
}
