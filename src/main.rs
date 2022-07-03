use life::{build_neighbors, Cell};
use minifb::{Key, Window, WindowOptions};
const WIDTH: usize = 360;
const HEIGHT: usize = 180;

fn main() {
    let mut buffer: Vec<u32>;
    let mut cells: Vec<Cell<bool>> = Vec::with_capacity(WIDTH * HEIGHT);
    for _i in 0..WIDTH * HEIGHT {
        cells.push(Cell {
            value: fastrand::bool(),
            neighbors: Vec::new(),
        })
    }

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        build_neighbors(&mut cells, WIDTH);
        cells = cells.iter().map(|a| a.dry_life_update()).collect();
        buffer = cells
            .iter()
            .map(|a| a.draw())
            .collect();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
