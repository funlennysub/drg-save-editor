use editor_core::save_file::SaveFile;
use leptos::{html::Input, *};
use web_sys::Event;

#[component]
pub(crate) fn FileUpload(
    cx: Scope,
    set_file_name: WriteSignal<Option<String>>,
    set_save_file: WriteSignal<Option<SaveFile>>,
) -> impl IntoView {
    let file_input = create_node_ref::<Input>(cx);
    let on_file_change = move |_ev: Event| {
        if let Some(files) = file_input.get().and_then(|fi| fi.files()) {
            let file = files.get(0).unwrap();

            set_file_name.update(|n| *n = Some(file.name()));

            let file_blob_promise = js_sys::Promise::resolve(&file.array_buffer());
            spawn_local(async move {
                let bytes = wasm_bindgen_futures::JsFuture::from(file_blob_promise)
                    .await
                    .unwrap();
                let byte_arr = js_sys::Uint8Array::new(&bytes);
                let buffer = &byte_arr.to_vec()[..];

                let save_file = SaveFile::from_bytes(buffer).expect("valid save file");
                set_save_file.update(|n| *n = Some(save_file));
            })
        }
    };

    view! { cx, <input type="file" accept=".sav" on:change=on_file_change node_ref=file_input/> }
}
