use super::device_card::EmptyDevices;
use crate::components::devices::device_card::{Device, DeviceCard};
use leptos::prelude::*;

// Dummy server functions
#[server(GetDevices)]
pub async fn get_devices() -> Result<Vec<Device>, ServerFnError> {
    // TODO server fn
    Ok(vec![
        Device {
            id: 1,
            name: "ESP32".to_string(),
            device_type: "Microcontroller".to_string(),
            status: "online".to_string(),
            last_seen: Some("Just now".to_string()),
        },
        Device {
            id: 2,
            name: "Camera".to_string(),
            device_type: "Surveillance".to_string(),
            status: "online".to_string(),
            last_seen: Some("2 minutes ago".to_string()),
        },
        Device {
            id: 3,
            name: "Chicken nugget bombaclat".to_string(),
            device_type: "KFC".to_string(),
            status: "offline".to_string(),
            last_seen: Some("3 hours ago".to_string()),
        },
    ])
}

#[server(AddDevice)]
pub async fn add_device(name: String, device_type: String) -> Result<Device, ServerFnError> {
    // TODO server fn
    Ok(Device {
        id: 999,
        name,
        device_type,
        status: "online".to_string(),
        last_seen: Some("Just now".to_string()),
    })
}

#[component]
pub fn Devices() -> impl IntoView {
    let show_add = RwSignal::new(false);
    let device_name = RwSignal::new(String::new());
    let device_type = RwSignal::new(String::from("ESP32"));
    let error = RwSignal::new(None::<String>);

    let devices = Resource::new(|| (), |_| async { get_devices().await });

    let add_action = Action::new(move |_: &()| {
        let name = device_name.get().trim().to_string();
        let kind = device_type.get();

        async move {
            if name.is_empty() {
                return Err(ServerFnError::new("Device name is required"));
            }

            add_device(name, kind).await
        }
    });

    Effect::new(move |_| {
        let result = add_action.value().get();

        match result {
            Some(Ok(_)) => {
                device_name.set(String::new());
                error.set(None);
                show_add.set(false);
                devices.refetch();
            }

            Some(Err(err)) => {
                error.set(Some(err.to_string()));
            }

            None => {}
        }
    });

    view! {
        <section class="
            min-h-screen w-full
            flex-1
            bg-slate-950
            px-4 py-8
            sm:px-6
            lg:px-8
        ">
            <div class="mx-auto w-full max-w-7xl">

                <div class="
                    mb-8
                    flex flex-col gap-5
                    sm:flex-row
                    sm:items-end
                    sm:justify-between
                ">
                    <div>
                        <div class="
                            mb-2
                            text-xs font-semibold
                            uppercase tracking-[0.18em]
                            text-slate-500
                        ">
                            "Account"
                        </div>

                        <h1 class="
                            text-3xl font-semibold
                            tracking-tight
                            text-slate-100
                            sm:text-4xl
                        ">
                            "Your devices"
                        </h1>

                        <p class="
                            mt-2
                            text-sm
                            text-slate-400
                        ">
                            "Manage the devices connected to your account."
                        </p>
                    </div>

                    <button
                        type="button"
                        class="
                            inline-flex
                            items-center
                            justify-center
                            gap-2
                            rounded-xl
                            border
                            border-blue-400/20
                            bg-blue-500/10
                            px-4 py-2.5
                            text-sm font-medium
                            text-blue-300
                            transition
                            hover:border-blue-400/30
                            hover:bg-blue-500/20
                            hover:text-blue-200
                        "
                        on:click=move |_| {
                            show_add.update(|value| *value = !*value);
                            error.set(None);
                        }
                    >
                        <span class="text-lg leading-none">"+"</span>
                        <span>"Add device"</span>
                    </button>
                </div>

                <Show when=move || show_add.get()>
                    <div class="
                        mb-6
                        rounded-2xl
                        border
                        border-slate-800
                        bg-slate-900/70
                        p-5
                        shadow-xl
                    ">
                        <div class="mb-5">
                            <h2 class="
                                text-base font-semibold
                                text-slate-100
                            ">
                                "Add a device"
                            </h2>

                            <p class="
                                mt-1
                                text-sm
                                text-slate-500
                            ">
                                "Connect another device to your account."
                            </p>
                        </div>

                        <div class="
                            grid
                            gap-4
                            lg:grid-cols-[1fr_220px_auto]
                            lg:items-end
                        ">
                            <div>
                                <label
                                    for="device-name"
                                    class="
                                        mb-2
                                        block
                                        text-xs font-medium
                                        text-slate-400
                                    "
                                >
                                    "Device name"
                                </label>

                                <input
                                    id="device-name"
                                    type="text"
                                    placeholder="e.g. Work laptop"
                                    class="
                                        w-full
                                        rounded-xl
                                        border
                                        border-slate-700
                                        bg-slate-950
                                        px-4 py-3
                                        text-sm
                                        text-slate-100
                                        outline-none
                                        placeholder:text-slate-600
                                        focus:border-blue-500/50
                                        focus:ring-2
                                        focus:ring-blue-500/10
                                    "
                                    prop:value=move || device_name.get()
                                    on:input=move |event| {
                                        device_name.set(
                                            event_target_value(&event)
                                        );
                                    }
                                />
                            </div>

                            <div>
                                <label
                                    for="device-type"
                                    class="
                                        mb-2
                                        block
                                        text-xs font-medium
                                        text-slate-400
                                    "
                                >
                                    "Type"
                                </label>

                                <select
                                    id="device-type"
                                    class="
                                        w-full
                                        rounded-xl
                                        border
                                        border-slate-700
                                        bg-slate-950
                                        px-4 py-3
                                        text-sm
                                        text-slate-100
                                        outline-none
                                        focus:border-blue-500/50
                                        focus:ring-2
                                        focus:ring-blue-500/10
                                    "
                                    prop:value=move || device_type.get()
                                    on:change=move |event| {
                                        device_type.set(
                                            event_target_value(&event)
                                        );
                                    }
                                >
                                    <option value="Laptop">"Laptop"</option>
                                    <option value="Desktop">"Desktop"</option>
                                    <option value="Mobile">"Mobile"</option>
                                    <option value="Tablet">"Tablet"</option>
                                    <option value="Server">"Server"</option>
                                    <option value="Other">"Other"</option>
                                </select>
                            </div>

                            <button
                                type="button"
                                class="
                                    rounded-xl
                                    bg-blue-600
                                    px-5 py-3
                                    text-sm font-medium
                                    text-white
                                    transition
                                    hover:bg-blue-500
                                    disabled:cursor-not-allowed
                                    disabled:opacity-50
                                "
                                disabled=move || add_action.pending().get()
                                on:click=move |_| {
                                    error.set(None);
                                    add_action.dispatch(());
                                }
                            >
                                {move || {
                                    if add_action.pending().get() {
                                        "Adding..."
                                    } else {
                                        "Add device"
                                    }
                                }}
                            </button>
                        </div>

                        <Show when=move || error.get().is_some()>
                            <div class="
                                mt-4
                                rounded-lg
                                border
                                border-red-500/20
                                bg-red-500/5
                                px-3 py-2
                                text-sm
                                text-red-300
                            ">
                                {move || error.get().unwrap_or_default()}
                            </div>
                        </Show>
                    </div>
                </Show>

                <Suspense
                    fallback=move || {
                        view! {
                            <div class="
                                flex
                                min-h-[280px]
                                items-center
                                justify-center
                                rounded-2xl
                                border
                                border-slate-800
                                bg-slate-900/40
                            ">
                                <div class="
                                    flex
                                    items-center
                                    gap-3
                                    text-sm
                                    text-slate-500
                                ">
                                    <div class="
                                        h-5 w-5
                                        animate-spin
                                        rounded-full
                                        border-2
                                        border-slate-700
                                        border-t-slate-400
                                    "></div>

                                    <span>
                                        "Loading your devices..."
                                    </span>
                                </div>
                            </div>
                        }
                    }
                >
                    {move || {
                        match devices.get() {
                            Some(Ok(items)) => {
                                if items.is_empty() {
                                    view! {
                                        <EmptyDevices
                                            on_add=Callback::new(move |_| {
                                                show_add.set(true);
                                            })
                                        />
                                    }
                                    .into_any()
                                } else {
                                    view! {
                                        <div class="
                                            grid
                                            grid-cols-1
                                            gap-4
                                            sm:grid-cols-2
                                            xl:grid-cols-3
                                        ">
                                            <For
                                                each=move || items.clone()
                                                key=|device| device.id
                                                let:device
                                            >
                                                <DeviceCard device=device />
                                            </For>
                                        </div>
                                    }
                                    .into_any()
                                }
                            }

                            Some(Err(err)) => {
                                view! {
                                    <div class="
                                        rounded-2xl
                                        border
                                        border-red-500/20
                                        bg-red-500/5
                                        p-6
                                    ">
                                        <h2 class="
                                            text-sm font-semibold
                                            text-red-300
                                        ">
                                            "Unable to load devices"
                                        </h2>

                                        <p class="
                                            mt-2
                                            text-sm
                                            text-slate-500
                                        ">
                                            {err.to_string()}
                                        </p>
                                    </div>
                                }
                                .into_any()
                            }

                            None => {
                                view! {
                                    <div></div>
                                }
                                .into_any()
                            }
                        }
                    }}
                </Suspense>
            </div>
        </section>
    }
}
