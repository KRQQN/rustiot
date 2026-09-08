use crate::{
    components::devices::{
        add_device::AddDevice,
        device_card::{DeviceCard, EmptyDevices},
    },
    server_fns::devices::get_devices,
};
use leptos::prelude::*;

#[component]
pub fn Devices() -> impl IntoView {
    let show_add = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let devices = Resource::new(|| (), |_| async { get_devices().await });

    view! {
        <section class="
            min-h-screen
            w-full
            flex-1
            bg-slate-950
            px-4
            py-8
            sm:px-6
            lg:px-8
        ">
            <div class="mx-auto w-full max-w-7xl">

                <div class="
                    mb-8
                    flex
                    flex-col
                    gap-5
                    sm:flex-row
                    sm:items-end
                    sm:justify-between
                ">
                    <div>
                        <div class="
                            mb-2
                            text-xs
                            font-semibold
                            uppercase
                            tracking-[0.18em]
                            text-slate-500
                        ">
                            "Account"
                        </div>

                        <h1 class="
                            text-3xl
                            font-semibold
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
                            px-4
                            py-2.5
                            text-sm
                            font-medium
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
                    <AddDevice
                        on_success=Callback::new(move |_| {
                            show_add.set(false);
                            devices.refetch();
                        })
                    />
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
                                        h-5
                                        w-5
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
                                            text-sm
                                            font-semibold
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
