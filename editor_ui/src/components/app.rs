use crate::components::{file_upload::FileUpload, save_display::SaveDisplay};
use editor_core::save_file::SaveFile;
use leptos::*;

#[component]
pub(crate) fn App(cx: Scope) -> impl IntoView {
    let (_, set_file_name) = create_signal(cx, None::<String>);
    let save_file = create_rw_signal(cx, None::<SaveFile>);

    provide_context(cx, save_file);

    view! { cx,
        <Show
            when=move || save_file.with(Option::is_none)
            fallback=move |cx| {
                view! { cx, <SaveDisplay/> }
            }
        >
            <FileUpload set_file_name set_save_file=save_file.write_only()/>
        </Show>
    }
}
