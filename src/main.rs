use leptos::*;

fn main() {
    mount_to_body(|cx| view!{ cx, <App/>> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let clear = move |_ev| set_count.set(0);
    let decrement = move |_ev| set_count.update(|count| *count -= 1);
    let increment = move |_ev| set_count.update(|count| *count += 1);
    
    view! {
        cx,
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {move || count.get().to_string()} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}