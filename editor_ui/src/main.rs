mod components;

use components::file_upload::*;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx, <FileUpload/> })
}
