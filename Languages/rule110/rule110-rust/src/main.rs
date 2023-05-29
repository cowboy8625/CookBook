use std::time::Duration;
const WIDTH: usize = 100;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Allive,
    Dead,
}

fn main() {
    let mut grid = vec![Cell::Dead; WIDTH];
    grid[WIDTH - 1] = Cell::Allive;
    loop {
        display(&grid);
        grid = update(grid);
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn update(old: Vec<Cell>) -> Vec<Cell> {
    let len = old.len();
    (0..WIDTH)
        .map(|i| {
            let a = old[i.wrapping_sub(1).rem_euclid(len)];
            let b = old[i];
            let c = old[i.wrapping_add(1).rem_euclid(len)];
            is_alive(a, b, c)
        })
        .collect()
}

fn is_alive(a: Cell, b: Cell, c: Cell) -> Cell {
    match (a, b, c) {
        (Cell::Allive, Cell::Allive, Cell::Allive) => Cell::Dead, // 7
        (Cell::Allive, Cell::Allive, Cell::Dead) => Cell::Allive, // 6
        (Cell::Allive, Cell::Dead, Cell::Allive) => Cell::Allive, // 5
        (Cell::Allive, Cell::Dead, Cell::Dead) => Cell::Dead,     // 4
        (Cell::Dead, Cell::Allive, Cell::Allive) => Cell::Allive, // 3
        (Cell::Dead, Cell::Allive, Cell::Dead) => Cell::Allive,   // 2
        (Cell::Dead, Cell::Dead, Cell::Allive) => Cell::Allive,   // 1
        (Cell::Dead, Cell::Dead, Cell::Dead) => Cell::Dead,       // 0
    }
}

fn display(grid: &[Cell]) {
    for cell in grid.iter() {
        match cell {
            Cell::Allive => print!("#"),
            Cell::Dead => print!(" "),
        }
    }
    println!();
}
