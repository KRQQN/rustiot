// use crate::server_fns::get_device_data::{get_device_data, DeviceReading};
// use leptos::prelude::*;
//
// #[component]
// pub fn InfluxToast() -> impl IntoView {
//     let (visible, set_visible) = signal(false);
//     let (loading, set_loading) = signal(false);
//     let (error, set_error) = signal::<Option<String>>(None);
//     let (data, set_data) = signal::<Vec<DeviceReading>>(vec![]);
//
//     let load = move |_| {
//         set_visible.set(true);
//         set_loading.set(true);
//         set_error.set(None);
//         set_data.set(vec![]);
//
//         leptos::task::spawn_local(async move {
//             match get_device_data("esp-rustiot".into(), Some(10)).await {
//                 Ok(readings) => {
//                     set_data.set(readings);
//                     set_loading.set(false);
//                 }
//                 Err(e) => {
//                     set_error.set(Some(e.to_string()));
//                     set_loading.set(false);
//                 }
//             }
//         });
//     };
//
//     let close = move |_| {
//         set_visible.set(false);
//     };
//
//     view! {
//         <button
//             class="fixed bottom-6 right-20 z-50 px-4 py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg shadow-lg"
//             on:click=load
//         >
//             "Test InfluxDB"
//         </button>
//
//         <Show when=move || visible.get()>
//             <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60">
//                 <div class="bg-neutral-800 text-white rounded-xl shadow-2xl w-full max-w-md mx-4 overflow-hidden">
//                     // Header
//                     <div class="flex items-center justify-between px-5 py-4 border-b border-neutral-700">
//                         <h3 class="text-lg font-semibold">"InfluxDB Test"</h3>
//                         <button
//                             class="text-neutral-400 hover:text-white text-xl leading-none"
//                             on:click=close
//                         >
//                             "×"
//                         </button>
//                     </div>
//
//                     <div class="px-5 py-4 max-h-80 overflow-y-auto">
//                         <Show when=move || loading.get()>
//                             <p class="text-neutral-400">"Loading..."</p>
//                         </Show>
//
//                         <Show when=move || error.get().is_some()>
//                             <p class="text-red-400">
//                                 {move || error.get().unwrap_or_default()}
//                             </p>
//                         </Show>
//
//                         <Show when=move || !loading.get() && error.get().is_none()>
//                             <Show
//                                 when=move || data.get().is_empty()
//                                 fallback=move || {
//                                     view! {
//                                         <ul class="space-y-2 text-sm">
//                                             <For
//                                                 each=move || data.get()
//                                                 key=|r| format!("{}", r.time)
//                                                 children=move |r| {
//                                                     view! {
//                                                     <li class="flex justify-between border-b border-neutral-700 pb-1">
//                                                         <span class="text-neutral-400">{r.time}</span>
//                                                         <span class="font-mono">{r.device_id}</span>
//                                                         <span>
//                                                             "H: " {r.humidity} "% "
//                                                             "T: " {r.temperature} "°C"
//                                                         </span>
//                                                     </li>}
//                                                 }
//                                             />
//                                         </ul>
//                                     }
//                                 }
//                             >
//                                 <p class="text-neutral-400">"No data returned"</p>
//                             </Show>
//                         </Show>
//                     </div>
//
//                     <div class="px-5 py-3 border-t border-neutral-700 flex justify-end">
//                         <button
//                             class="px-4 py-1.5 bg-neutral-700 hover:bg-neutral-600 rounded-lg text-sm"
//                             on:click=close
//                         >
//                             "Close"
//                         </button>
//                     </div>
//                 </div>
//             </div>
//         </Show>
//     }
// }
