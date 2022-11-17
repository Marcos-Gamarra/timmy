pub trait Input {
    fn insert_character(&mut self, character: char);
    fn tab(&mut self, _number_of_spaces: u16) {}
    fn enter(&mut self) {}
    fn backspace(&mut self) {}
    fn right(&mut self) {}
    fn left(&mut self) {}
    fn up(&mut self) {}
    fn down(&mut self) {}
}


