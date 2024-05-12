use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos::ev::Event;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use crate::cleanup::remove_comments;
use crate::keywords::get_keywords;

#[derive(Serialize, Deserialize)]
pub struct Clean {
    base: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
#[derive(Serialize, Deserialize)]
struct PathArgs<'a> {
    path: &'a str,
}
#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (file_input, set_file_input) = create_signal(String::new());
    let (file_output, set_file_output) = create_signal(String::new());
    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let update_filtered = move |ev: Event| {
        let v = event_target_value(&ev).clone();
        info!("Obtained etv: {v}");
        if !v.is_empty() {
            let filtered = remove_comments(v);
            // info!("Obtained filtered text: {v}");
            set_file_output.set(filtered);
        }
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let mut name = name.get_untracked();
            if name.is_empty() {
                name = "/".to_owned();
            }

            let args = to_value(&PathArgs { path: &name }).unwrap();
            let fs: JsValue = invoke("read", args).await;
            let fs: Clean = from_value(fs).unwrap();
            if !fs.base.is_empty() {
                let filtered = remove_comments(fs.base.clone());
                set_file_output.set(filtered);
            }
            set_file_input.set(fs.base);
        });
    };

    view! {
        <main class="container">
            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a file directory..."
                    on:input=update_name
                />
                <button type="submit">"Open"</button>
            </form>
            <div class="row">
                <textarea
                    class="display_text"
                    prop:value=move|| file_input.get()
                    on:input=update_filtered>
                </textarea>
                <div class="display_text"
                    prop:value=move || file_output.get()>
                    <code>{file_output}</code>
                </div>
            </div>
        </main>
    }
}
