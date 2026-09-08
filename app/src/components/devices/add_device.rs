use crate::server_fns::devices::add_device;
use leptos::prelude::*;
use serde_json::{Map, Value};
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ConfigValueType {
    Text,
    Number,
    Boolean,
    Timestamp,
}

impl ConfigValueType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::Timestamp => "timestamp",
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "number" => Self::Number,
            "boolean" => Self::Boolean,
            "timestamp" => Self::Timestamp,
            _ => Self::Text,
        }
    }
}

#[derive(Clone)]
struct ConfigRow {
    id: Uuid,
    key: RwSignal<String>,
    value_type: RwSignal<ConfigValueType>,
}

#[component]
pub fn AddDevice(on_success: Callback<()>) -> impl IntoView {
    let device_name = RwSignal::new(String::new());
    let device_type = RwSignal::new(String::from("Laptop"));
    let error = RwSignal::new(None::<String>);

    let config_rows = RwSignal::new(vec![ConfigRow {
        id: Uuid::new_v4(),
        key: RwSignal::new(String::new()),
        value_type: RwSignal::new(ConfigValueType::Text),
    }]);

    let add_action = Action::new(move |_: &()| {
        let name = device_name.get().trim().to_string();
        let kind = device_type.get();
        let mut config = Map::new();

        for row in config_rows.get() {
            let key = row.key.get().trim().to_string();
            if key.is_empty() {
                continue;
            }

            config.insert(
                key,
                Value::String(row.value_type.get().as_str().to_string()),
            );
        }

        let config = Value::Object(config);

        async move {
            if name.is_empty() {
                return Err(ServerFnError::new("Device name is required"));
            }
            add_device(name, kind, config).await
        }
    });

    Effect::new(move |_| match add_action.value().get() {
        Some(Ok(_)) => {
            device_name.set(String::new());
            error.set(None);
            config_rows.set(vec![ConfigRow {
                id: Uuid::new_v4(),
                key: RwSignal::new(String::new()),
                value_type: RwSignal::new(ConfigValueType::Text),
            }]);
            on_success.run(());
        }
        Some(Err(err)) => {
            error.set(Some(err.to_string()));
        }
        None => {}
    });

    let add_empty_row_if_needed = move || {
        config_rows.update(|rows| {
            let last_is_filled = rows
                .last()
                .map(|row| !row.key.get().trim().is_empty())
                .unwrap_or(false);

            if last_is_filled {
                rows.push(ConfigRow {
                    id: Uuid::new_v4(),
                    key: RwSignal::new(String::new()),
                    value_type: RwSignal::new(ConfigValueType::Text),
                });
            }
        });
    };

    view! {
        <div class="mb-6 rounded-2xl border border-slate-800 bg-slate-900/60 p-6">
            <div class="mb-6">
                <h2 class="text-lg font-semibold text-slate-100">"Add device"</h2>
                <p class="mt-1 text-sm text-slate-500">"Configure your new device."</p>
            </div>

            <div class="space-y-5">
                <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
                    <div>
                        <label class="mb-2 block text-sm font-medium text-slate-300">
                            "Device name"
                        </label>
                        <input
                            type="text"
                            placeholder="My laptop"
                            class="w-full rounded-xl border border-slate-700 bg-slate-950 px-4 py-2.5 text-sm text-slate-100 outline-none placeholder:text-slate-600 focus:border-blue-500/50"
                            prop:value=move || device_name.get()
                            on:input=move |ev| {
                                device_name.set(event_target_value(&ev));
                            }
                        />
                    </div>

                    <div>
                        <label class="mb-2 block text-sm font-medium text-slate-300">
                            "Device type"
                        </label>
                        <select
                            class="w-full rounded-xl border border-slate-700 bg-slate-950 px-4 py-2.5 text-sm text-slate-100 outline-none focus:border-blue-500/50"
                            prop:value=move || device_type.get()
                            on:change=move |ev| {
                                device_type.set(event_target_value(&ev));
                            }
                        >
                            <option value="Laptop">"Laptop"</option>
                            <option value="Desktop">"Desktop"</option>
                            <option value="Mobile">"Mobile"</option>
                            <option value="Tablet">"Tablet"</option>
                            <option value="Server">"Server"</option>
                            <option value="Microcontroller">"Microcontroller"</option>
                            <option value="Camera">"Camera"</option>
                            <option value="Other">"Other"</option>
                        </select>
                    </div>
                </div>

                <div>
                    <div class="mb-3">
                        <label class="block text-sm font-medium text-slate-300">
                            "Configuration"
                        </label>
                        <p class="mt-1 text-xs text-slate-600">
                            "Declare the keys this device will report and their types."
                        </p>
                    </div>

                    <div class="mb-2 grid grid-cols-12 gap-2">
                        <div class="col-span-7 px-3 text-xs font-medium uppercase tracking-wide text-slate-600">
                            "Key"
                        </div>
                        <div class="col-span-5 px-3 text-xs font-medium uppercase tracking-wide text-slate-600">
                            "Type"
                        </div>
                    </div>

                    <div class="space-y-2">
                        <For
                            each=move || config_rows.get()
                            key=|row| row.id
                            children=move |row| {
                                view! {
                                    <div class="grid grid-cols-12 gap-2">
                                        <div class="col-span-7">
                                            <input
                                                type="text"
                                                placeholder="e.g. temperature, humidity..."
                                                class="w-full rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 outline-none placeholder:text-slate-600 focus:border-blue-500/50"
                                                prop:value=move || row.key.get()
                                                on:input=move |ev| {
                                                    row.key.set(event_target_value(&ev));
                                                    add_empty_row_if_needed();
                                                }
                                            />
                                        </div>

                                        <div class="col-span-5">
                                            <select
                                                class="w-full rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 outline-none focus:border-blue-500/50"
                                                prop:value=move || row.value_type.get().as_str()
                                                on:change=move |ev| {
                                                    let t = ConfigValueType::from_str(
                                                        &event_target_value(&ev)
                                                    );
                                                    row.value_type.set(t);
                                                }
                                            >
                                                <option value="text">"Text"</option>
                                                <option value="number">"Number"</option>
                                                <option value="boolean">"Boolean"</option>
                                                <option value="timestamp">"Timestamp"</option>
                                            </select>
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>

                {move || {
                    error.get().map(|message| {
                        view! {
                            <div class="rounded-lg border border-red-500/20 bg-red-500/5 px-3 py-2 text-sm text-red-300">
                                {message}
                            </div>
                        }
                    })
                }}

                <div class="flex justify-end">
                    <button
                        type="button"
                        class="rounded-xl bg-blue-500 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-blue-400"
                        on:click=move |_| {
                            add_action.dispatch(());
                        }
                    >
                        "Add device"
                    </button>
                </div>
            </div>
        </div>
    }
}
