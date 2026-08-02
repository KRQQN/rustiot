use crate::server_fns::login::Login;
use leptos::prelude::*;

#[component]
pub fn LoginForm() -> impl IntoView {
    let login = ServerAction::<Login>::new();
    let value = login.value();
    let pending = login.pending();

    view! {
        <div class="w-full max-w-md">
            <div class="bg-black/60 backdrop-blur-xl border border-white/10 rounded-3xl p-8 shadow-2xl shadow-cyan-500/10">
                <div class="flex items-center gap-4 mb-8">
                    <div class="w-11 h-11 bg-white rounded-2xl flex items-center justify-center text-black font-bold text-2xl">
                        "N"
                    </div>
                    <div>
                        <div class="text-2xl font-semibold tracking-tighter text-white">"Sign In"</div>
                        <div class="text-slate-400 text-sm tracking-wide">"Welcome back to NEXTIOT"</div>
                    </div>
                </div>

                <ActionForm action=login attr:class="space-y-5">
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-slate-300 tracking-wide">
                            "Email"
                        </label>
                        <input
                            type="email"
                            name="email"
                            required
                            placeholder="you@company.com"
                            class="w-full px-4 py-3.5 bg-white/5 border border-white/10 rounded-2xl text-white placeholder:text-slate-500 focus:outline-none focus:border-cyan-500/60 focus:ring-2 focus:ring-cyan-500/20 transition-all"
                        />
                    </div>

                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-slate-300 tracking-wide">
                            "Password"
                        </label>
                        <input
                            type="password"
                            name="password"
                            required
                            placeholder="Your password"
                            class="w-full px-4 py-3.5 bg-white/5 border border-white/10 rounded-2xl text-white placeholder:text-slate-500 focus:outline-none focus:border-cyan-500/60 focus:ring-2 focus:ring-cyan-500/20 transition-all"
                        />
                    </div>

                    <button
                        type="submit"
                        class="w-full mt-2 px-6 py-4 bg-cyan-500 hover:bg-cyan-400 disabled:bg-cyan-500/50 disabled:cursor-not-allowed text-black font-semibold text-lg rounded-2xl transition-all duration-300 shadow-lg shadow-cyan-500/20 hover:shadow-cyan-400/30"
                        disabled=pending
                    >
                        {move || if pending.get() { "Signing in..." } else { "Login" }}
                    </button>
                </ActionForm>

                <div class="mt-6 min-h-[1.5rem]">
                    {move || match value.get() {
                        Some(Ok(())) => view! {
                            <p class="text-cyan-400 text-sm text-center font-medium">
                                "Logged in successfully!"
                            </p>
                        }.into_any(),
                        Some(Err(err)) => view! {
                            <p class="text-red-400 text-sm text-center">
                                {err.to_string()}
                            </p>
                        }.into_any(),
                        None => view! { <p></p> }.into_any(),
                    }}
                </div>
            </div>
        </div>
    }
}
