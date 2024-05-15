mod app;
pub mod cleanup;
mod keywords;
mod descriptors;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
