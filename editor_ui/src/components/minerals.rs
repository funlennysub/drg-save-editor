use editor_core::{
    registry::{MAX_F32, MAX_I32},
    save_file::minerals::Minerals,
};
use leptos::*;

#[component]
fn ResourceInput(cx: Scope, item: String, id: u8) -> impl IntoView {
    let (minerals, set_minerals) =
        use_context::<(Signal<Option<Minerals>>, SignalSetter<(u8, f32)>)>(cx).expect("save file");

    let value = move || {
        {
            minerals.get().map(|m| match id {
                0 => m.magnite,
                1 => m.bismor,
                2 => m.croppa,
                3 => m.umanite,
                4 => m.jadiz,
                5 => m.enor_pearl,
                _ => unreachable!(),
            })
        }
        .map(|v| format!("{v:.0}"))
    };

    let on_input = move |ev| {
        let val = event_target_value(&ev);
        let num = val.parse::<f32>().unwrap_or_default();
        let num_clamped = num.clamp(0.0, MAX_F32);
        set_minerals.set((id, num_clamped));
    };

    view! { cx,
        <div>
        <p>{value}</p>
            <label>{item}</label>
            <input
                type="number"
                min=0
                max=MAX_I32
                prop:value=value
                on:change=on_input
            />
        </div>
    }
}

#[component]
pub(crate) fn Minerals(cx: Scope) -> impl IntoView {
    view! { cx,
        <input type="number" min=0 max=100/>
        <ResourceInput item=String::from("Magnite") id=0/>
        <ResourceInput item=String::from("Bismor") id=1/>
        <ResourceInput item=String::from("Croppa") id=2/>
        <ResourceInput item=String::from("Umanite") id=3/>
        <ResourceInput item=String::from("Jadiz") id=4/>
        <ResourceInput item=String::from("Enor pearl") id=5/>
    }
}
