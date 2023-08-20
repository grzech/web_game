use leptos::*;

fn main() {
    mount_to_body(|cx| view!{ cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (game_started, set_game_started) = create_signal(cx, false);
    let start_game = move |_ev| { set_game_started.set(true) };
    view! {
        cx,
        <div id="Web Game">
            <h4>"Player name"</h4>
            <h3><input type="text"></input></h3>
            <h3><button on:click=start_game>"Start"</button></h3>
            <Show
                when= move || { game_started.get() }
                fallback=|_cx| view! { cx, <> }
            >
                <Game/>
            </Show>
        </div>
    }
    
}

#[component]
fn Game(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div id="gameboard">
            <div class="boardrow">
                <div class="cell"></div>
                <div class="cell"> </div>
                <div class="cell"></div>
                <div class="cell"></div>
                <div class="cell"></div>
            </div>
            <div class="boardrow">
                <div class="cell snakebody"></div>
                <div class="cell"> </div>
                <div class="cell snakehead"></div>
                <div class="cell snakebody"></div>
                <div class="cell food"></div>
            </div>
            <div class="boardrow">
                <div class="cell"></div>
                <div class="cell"> </div>
                <div class="cell"></div>
                <div class="cell"></div>
                <div class="cell"></div>
            </div>
        </div>
    }
}