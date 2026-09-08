use leptos::prelude::*;

use crate::server_fns::devices::add_device;

#[component]
pub fn AddDevice(on_success: Callback<()>) -> impl IntoView {
    let device_name = RwSignal::new(String::new());
    let device_type = RwSignal::new(String::from("Laptop"));
    let error = RwSignal::new(None::<String>);

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

    Effect::new(move |_| match add_action.value().get() {
        Some(Ok(_)) => {
            device_name.set(String::new());
            error.set(None);
            on_success.run(());
        }

        Some(Err(err)) => {
            error.set(Some(err.to_string()));
        }

        None => {}
    });

    view! {

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
                                text-base
                                font-semibold
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
                                        text-xs
                                        font-medium
                                        text-slate-400
                                    "
                                >
                                    "Device name"
                                </label>

                                <input
                                    id="device-name"
                                    type="text"
                                    placeholder="e.g. ESP32 Living Room"
                                    class="
                                        w-full
                                        rounded-xl
                                        border
                                        border-slate-700
                                        bg-slate-950
                                        px-4
                                        py-3
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
                                        text-xs
                                        font-medium
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
                                        px-4
                                        py-3
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
                                    <option value="Microcontroller">
                                        "Microcontroller"
                                    </option>
                                    <option value="Camera">"Camera"</option>
                                    <option value="Other">"Other"</option>
                                </select>
                            </div>

                            <button
                                type="button"
                                class="
                                    rounded-xl
                                    bg-blue-600
                                    px-5
                                    py-3
                                    text-sm
                                    font-medium
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
                                px-3
                                py-2
                                text-sm
                                text-red-300
                            ">
                                {move || error.get().unwrap_or_default()}
                            </div>
                        </Show>
                    </div>



    }
}
