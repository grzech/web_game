use leptos::*;

#[derive(Clone, Copy)]
pub enum SnakeFields {
    Head,
    Food,
    Body,
    Empty,
}

#[derive(Clone)]
struct BoardRow (Vec<SnakeFields>);

impl std::ops::Deref for BoardRow {
    type Target = [SnakeFields];
    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

pub struct GameBoard (Vec<BoardRow>);

impl GameBoard {
    pub fn new((rows, cols): (usize, usize)) -> GameBoard {
        let row = BoardRow(vec![SnakeFields::Empty; cols]);
        GameBoard(vec![row; rows])
    }

    fn get_rows(&self) -> &[BoardRow] {
        &self.0[..]
    }
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
pub fn Row<'a>(cx: Scope, cells: &'a [SnakeFields]) -> impl IntoView {
    view! {
        cx,
        <div class="boardrow">
            { cells.into_iter()
              .map(|cell_type| { view! { cx, <Cell state=*cell_type/> } } )
              .collect::<Vec<_>>() }
        </div>
    }
}

#[component]
pub fn Board<'a>(cx: Scope, board: &'a GameBoard) -> impl IntoView {
    view! {
        cx,
        <div id="gameboard">
            { board.get_rows().into_iter()
              .map(|row| { view! { cx, <Row cells=row/> } } )
              .collect::<Vec<_>>() }
        </div>
    }
}