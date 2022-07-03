

pub struct Cell<T> {
    pub value: T,
    pub neighbors: Vec<T>,
}

impl Cell<bool> {
    pub fn life_update(&self) -> Cell<bool> {
        let living_neighbors = self.neighbors.iter().filter(|a| **a).count();
        let value = (self.value && living_neighbors == 2) || living_neighbors == 3;
        Cell {
            value: value,
            neighbors: Vec::new(),
        }
    }
    pub fn draw(&self) -> u32{
        if self.value {
            u32::MAX
        } else {
            0
        }
        
    }
    pub fn dry_life_update(&self) -> Cell<bool> {
        let living_neighbors = self.neighbors.iter().filter(|a| **a).count();
        let value = (self.value && living_neighbors == 2) || (living_neighbors == 3 || (living_neighbors == 7 && ! self.value));
        Cell {
            value: value,
            neighbors: Vec::new(),
        }
    }
    
}
pub fn build_neighbors(cells: &mut Vec<Cell<bool>>, WIDTH: usize) {
    for i in 0..cells.len() {
        let mut neighbors: Vec<bool> = Vec::new();
        let x = (i % WIDTH) as isize;
        let y = (i / WIDTH) as isize;
        for x_adj in [-1, 0, 1] {
            for y_adj in [-1, 0, 1] {
                if !(y_adj == 0 && x_adj == 0) {
                    match cells.get(((x + x_adj) + (y + y_adj) * WIDTH as isize) as usize) {
                        Some(a) => neighbors.push(a.value),
                        None => (),
                    }
                }
            }
        }
        cells[i].neighbors = neighbors;
    }
}
