use editor_core::save_file::{SaveFile, minerals::Mineral};
use leptos::*;

use crate::components::minerals::Minerals;

#[component]
pub(crate) fn SaveDisplay(cx: Scope) -> impl IntoView {
    let save_file = use_context::<RwSignal<Option<SaveFile>>>(cx).expect("save file");
    let minerals = create_slice(
        cx,
        save_file,
        |s| s.as_ref().map(|s| s.minerals),
        |s, n: (u8, f32)| {
            s.as_mut().map(|mut s| {
                match n.0 {
                    0 => s.minerals.magnite = n.1,
                    1 => s.minerals.bismor = n.1,
                    2 => s.minerals.croppa = n.1,
                    3 => s.minerals.umanite = n.1,
                    4 => s.minerals.jadiz = n.1,
                    5 => s.minerals.enor_pearl = n.1,
                    _ => {}
                };
                s
            });
        },
    );
    provide_context(cx, minerals);

    view! { cx, <Minerals/> }
}
