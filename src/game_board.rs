use leptos::*;

#[derive(Clone, Copy)]
pub enum SnakeFields {
    Head,
    Food,
    Body,
    Empty,
}

#[component]
pub fn Cell(cx: Scope, state: SnakeFields) -> impl IntoView {
    let cell_type = match state {
        SnakeFields::Head => "snakehead",
        SnakeFields::Food => "food",
        SnakeFields::Body => "snakebody",
        SnakeFields::Empty => "",
    };

    view! {
            cx,
            <div class=move || {format!("cell {}", cell_type)}></div>}
}

#[component]
pub fn Row(cx: Scope, cells: &'static [SnakeFields]) -> impl IntoView {
    view! {
        cx,
        <div class="boardrow">
            { cells.into_iter()
              .map(|cell_type| { view! { cx, <Cell state=*cell_type/> } } )
              .collect::<Vec<_>>() }
        </div>
    }
}
