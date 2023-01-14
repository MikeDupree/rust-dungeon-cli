pub fn is_wall(row: u16, col: u16) -> bool {
    let size = termion::terminal_size().unwrap();
    row == 0 || row == size.1 - 4 || col == 0 || col == size.0 - 1
}
