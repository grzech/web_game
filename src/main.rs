use leptos::*;

#[derive(PartialEq, std::fmt::Debug)]
enum Game {
    None,
    Snake,
    PingPong,
}

static mut SELECTED_GAME: Game = Game::None;

fn main() {
    mount_to_body(|cx| view!{ cx, <App/>> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    unsafe{log!("App: Game = {:?}", SELECTED_GAME);};
    view! {
        cx,
        <div align="center">
            <Show
                when= move || { unsafe{ SELECTED_GAME == Game::None } }
                fallback=|cx| view! { cx, <Player/> }
            >
                <MainMenu/>
            </Show>
        </div>
    }
    
}


#[component]
fn MainMenu(cx: Scope) -> impl IntoView {
    let snake = move |_ev| unsafe { SELECTED_GAME = Game::Snake; };
    let ping_pong = move |_ev| unsafe { SELECTED_GAME = Game::PingPong; };
    unsafe{log!("MainMenu: Game = {:?}", SELECTED_GAME);};
    view! {
        cx,
        <div align="center">
            <h1>"Select the game"</h1>
            <h4><button on:click=snake>"Snake"</button></h4>
            <h4><button on:click=ping_pong>"Ping-Pong"</button></h4>
        </div>
    }

}

#[component]
fn Player(cx: Scope) -> impl IntoView {
    unsafe{log!("Player: Game = {:?}", SELECTED_GAME);};
    view! {
        cx,
        <div align="center">
            <h1>"Give your name"</h1>
            <h4><input type="text"></input></h4>
        </div>
    }
}