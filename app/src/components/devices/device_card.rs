use crate::server_fns::devices::remove_device;
use chrono::{DateTime, Utc};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub device_type: Option<String>,
    pub protocol: String,
    pub is_active: bool,
    pub last_seen_at: Option<DateTime<Utc>>,
}

#[component]
pub fn DeviceCard(device: Device, on_change: Callback<()>) -> impl IntoView {
    let menu_open = RwSignal::new(false);
    let is_online = device.is_active && device.last_seen_at.is_some();

    let status_dot = if is_online {
        "h-2 w-2 rounded-full bg-green-400"
    } else {
        "h-2 w-2 rounded-full bg-slate-600"
    };

    let status_label = if is_online {
        "Online"
    } else if !device.is_active {
        "Disabled"
    } else {
        "Offline"
    };

    let status_label_class = if is_online {
        "text-green-400"
    } else {
        "text-slate-500"
    };

    let device_type = device
        .device_type
        .clone()
        .unwrap_or_else(|| "Unknown".to_string());

    let last_seen = device
        .last_seen_at
        .map(|time| time.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| "Never".to_string());

    let icon = device_icon(&device_type);
    let device_id = device.id;

    let remove_action = Action::new(move |_: &()| {
        let id = device_id;
        async move { remove_device(id).await }
    });

    Effect::new(move |_| {
        if let Some(Ok(_)) = remove_action.value().get() {
            on_change.run(());
        }
    });

    Effect::new(move |_| {
        if let Some(Ok(_)) = remove_action.value().get() {
            on_change.run(());
        }
    });

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
                <div class="flex items-start justify-between">
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

                    <div class="flex items-center gap-3">
                        <div class="flex items-center gap-2 text-xs font-medium">
                            <span class=status_dot></span>
                            <span class=status_label_class>
                                {status_label}
                            </span>
                        </div>

                        <div class="relative">
                            <button
                                type="button"
                                class="
                                    flex
                                    h-8
                                    w-8
                                    items-center
                                    justify-center
                                    rounded-lg
                                    text-slate-500
                                    transition
                                    hover:bg-slate-800
                                    hover:text-slate-300
                                "
                                on:click=move |_| {
                                    menu_open.update(|v| *v = !*v);
                                }
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="16"
                                    height="16"
                                    viewBox="0 0 24 24"
                                    fill="currentColor"
                                >
                                    <circle cx="12" cy="5" r="1.5"/>
                                    <circle cx="12" cy="12" r="1.5"/>
                                    <circle cx="12" cy="19" r="1.5"/>
                                </svg>
                            </button>

                            <Show when=move || menu_open.get()>
                                <div class="
                                    absolute
                                    right-0
                                    top-9
                                    z-20
                                    w-40
                                    overflow-hidden
                                    rounded-xl
                                    border
                                    border-slate-700
                                    bg-slate-900
                                    shadow-xl
                                ">
                                    <button
                                        type="button"
                                        class="
                                            flex
                                            w-full
                                            items-center
                                            gap-2
                                            px-3
                                            py-2.5
                                            text-left
                                            text-sm
                                            text-red-400
                                            transition
                                            hover:bg-red-500/10
                                        "
                                        on:click=move |_| {
                                            menu_open.set(false);
                                            remove_action.dispatch(());
                                        }
                                    >
                                        "Remove device"
                                    </button>
                                </div>
                            </Show>
                        </div>
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
                    <p class="mt-1 text-xs text-slate-500">
                        {device_type}
                    </p>
                </div>

                <div class="mt-6 border-t border-slate-800 pt-4">
                    <div class="flex items-center justify-between">
                        <span class="text-xs text-slate-600">
                            "Protocol"
                        </span>
                        <span class="text-xs font-medium text-slate-500">
                            {device.protocol}
                        </span>
                    </div>
                    <div class="mt-2 flex items-center justify-between">
                        <span class="text-xs text-slate-600">
                            "Last seen"
                        </span>
                        <span class="text-xs font-medium text-slate-500">
                            {last_seen}
                        </span>
                    </div>
                </div>
            </div>
        </article>
    }
}

#[component]
pub fn EmptyDevices(on_change: Callback<()>) -> impl IntoView {
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
            <h2 class="text-base font-semibold text-slate-300">
                "No devices yet"
            </h2>
            <p class="mt-2 text-sm text-slate-500">
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
                on:click=move |_| on_change.run(())
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
