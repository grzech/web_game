use leptos::*;

#[derive(Clone, Copy, PartialEq, Debug)]
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

pub struct GameBoard {
    rows: Vec<BoardRow>,
    height: usize,
    width: usize,
}

impl GameBoard {
    pub fn new((rows, cols): (usize, usize)) -> GameBoard {
        let row = BoardRow(vec![SnakeFields::Empty; cols]);
        GameBoard{rows: vec![row; rows], height: rows, width: cols}
    }

    fn get_rows(&self) -> &[BoardRow] {
        &self.rows[..]
    }

    pub fn put_token(&mut self, (x, y) : (usize, usize), token: SnakeFields) -> Result<(), ()> {
        self.check_boundaries(x, y)?;
        self.rows[y].0[x] = token;
        Ok(())
    }

    pub fn get_token(&self, (x, y) : (usize, usize)) -> Result<SnakeFields, ()> {
        self.check_boundaries(x, y)?;
        Ok(self.rows[y].0[x])
    }

    fn check_boundaries(&self, x : usize, y : usize) -> Result<(), ()> {
        if x >= self.width || y >= self.height {
            return Err(());
        }
        Ok(())
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_put_and_get_token() {
        let mut brd = GameBoard::new((5, 5));
        let test_data = [
            ((1,1), SnakeFields::Body),
            ((2,2), SnakeFields::Body),
            ((2,1), SnakeFields::Head),
            ((2,3), SnakeFields::Food)];
        for (coordinates, token) in test_data {
            assert_eq!(brd.get_token(coordinates), Ok(SnakeFields::Empty));
            brd.put_token(coordinates, token).unwrap();
            assert_eq!(brd.get_token(coordinates), Ok(token));
        } 
    }
}