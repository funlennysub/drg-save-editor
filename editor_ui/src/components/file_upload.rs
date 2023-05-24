use editor_core::save_file::SaveFile;
use leptos::{html::Input, *};
use web_sys::Event;

#[component]
pub(crate) fn FileUpload(cx: Scope) -> impl IntoView {
    let file_input = create_node_ref::<Input>(cx);
    let on_file_change = move |_ev: Event| {
        if let Some(files) = file_input.get().map(|fi| fi.files()).flatten() {
            let file = files.get(0).unwrap();
            let file_blob_promise = js_sys::Promise::resolve(&file.array_buffer());
            spawn_local(async move {
                let bytes = wasm_bindgen_futures::JsFuture::from(file_blob_promise)
                    .await
                    .unwrap();
                let byte_arr = js_sys::Uint8Array::new(&bytes);
                let buffer = &byte_arr.to_vec()[..];
                let _ = SaveFile::from_bytes_rs(buffer);
                // let sf = SaveFile::from_bytes_rs(buffer).unwrap();
                // log!("{sf:?}");
            })
        }
    };

    view! { cx,
        <input type="file"
          on:change=on_file_change
          node_ref=file_input />
    }
}
