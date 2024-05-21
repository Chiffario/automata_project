use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos::ev::Event;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use crate::cleanup::{add_line_numbers, remove_comments};
use crate::{descriptors, keywords};

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
    let (descriptors, set_descriptors) = create_signal(String::new());
    let (pseudo, set_pseudo) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());

    let (identifiers, set_identifiers) = create_signal(String::new());
    let (keywords, set_keywords) = create_signal(String::new());
    let (separators, set_separators) = create_signal(String::new());
    let (strings, set_strings) = create_signal(String::new());
    let (consts, set_consts) = create_signal(String::new());
    let (operators, set_operators) = create_signal(String::new());


    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let update_filtered = move |ev: Event| {
        let v = event_target_value(&ev).clone();
        if !v.is_empty() {
            let filtered = remove_comments(v);
            let descr = keywords::count_tokens(filtered.clone());
            let table = match descr {
                Ok(val) => descriptors::create_descriptors(val),
                Err(e) => {
                    // set_error.set(e.to_string());
                    None
                },
            };
            let table = table.unwrap();
            set_descriptors.set(table.descriptors);
            set_pseudo.set(table.pseudocode);

            set_identifiers.set(table.identifiers);
            set_keywords.set(table.keywords);
            set_separators.set(table.separators);
            set_strings.set(table.strings);
            set_consts.set(table.consts);
            set_operators.set(table.operators);
            set_file_output.set(add_line_numbers(filtered));
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
                let filt_out = add_line_numbers(filtered.clone());
                let descr = keywords::count_tokens(filtered.clone());
                let table = match descr {
                    Ok(val) => descriptors::create_descriptors(val),
                    Err(e) => {
                        // set_error.set(e.to_string());
                        None
                    },
                };
                let table = table.unwrap();
                set_file_output.set(filt_out);
                set_descriptors.set(table.descriptors);
                set_pseudo.set(table.pseudocode);

                set_identifiers.set(table.identifiers);
                set_keywords.set(table.keywords);
                set_separators.set(table.separators);
                set_strings.set(table.strings);
                set_consts.set(table.consts);
                set_operators.set(table.operators);
            }
            set_file_input.set(fs.base);
        });
    };

    view! {
        <main class="container">
            <form class="row top" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a file directory..."
                    on:input=update_name
                />
                <button type="submit">"Open"</button>
            </form>
            <div class="row main">
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
            <div class="row bot">
                <div class="display_text"
                    prop:value=move || descriptors.get()>
                    <code style="max-width: 30%">{descriptors}</code>
                </div>
                <div class="display_text"
                    prop:value=move || pseudo.get()>
                    <code style="max-width: 30%">{pseudo}</code>
                </div>
                <div class="display_text"
                    prop:value=move || error.get()>
                    <code style="max-width: 30%">{error}</code>
                </div>
            </div>
            <div class="row bot">
                <div class="display_text"
                    prop:value=move || keywords.get()>
                    <code>10 - ключ. слова {keywords}</code>
                </div>
                <div class="display_text"
                    prop:value=move || identifiers.get()>
                    <code>20 - идентификаторы {identifiers}</code>
                </div>
                <div class="display_text"
                    prop:value=move || consts.get()>
                    <code>30 - числа {consts}</code>
                </div>
                <div class="display_text"
                    prop:value=move || operators.get()>
                    <code>40 - операторы{operators}</code>
                </div>
                <div class="display_text"
                    prop:value=move || strings.get()>
                    <code>50 - строки{strings}</code>
                </div>
                <div class="display_text"
                    prop:value=move || separators.get()>
                    <code>60 - разделители{separators}</code>
                </div>
            </div>
        </main>
    }
}
