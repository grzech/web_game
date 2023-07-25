use leptos::*;

fn main() {
    mount_to_body(|cx| view!{ cx, <App/>> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (value, set_value) = create_signal(cx, String::from(""));

    let clear = move |_ev| set_count.set(0);
    let decrement = move |_ev| set_count.update(|count| *count -= 1);
    let increment = move |_ev| set_count.update(|count| *count += 1);
    let enter_txt = move |ev| {
        let txt = event_target_value(&ev);
        if let Ok(data) = txt.parse::<i32>() {
            set_value.set(txt);
            log!("Entered text: \"{}\"", value.get());
            set_count.set(data);
        }
    };
    
    view! {
        cx,
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {move || count.get().to_string()} "!"</span>
            <button on:click=increment>"+1"</button>
            <input type="text" on:input=enter_txt></input>
        </div>
    }
}