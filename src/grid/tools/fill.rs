use crate::grid::{Cell, Grid};
use terminal::util::Point;

pub fn fill(grid: &mut Grid, point: Point, first_cell: Cell, fill_cell: Cell) {
    let cell = grid.get_mut_cell(point);

    if *cell == first_cell {
        *cell = fill_cell;
    } else {
        return;
    }

    if point.y != 0 {
        fill(
            grid,
            Point {
                y: point.y - 1,
                ..point
            },
            first_cell,
            fill_cell,
        );
    }
    if point.y < grid.size.height - 1 {
        fill(
            grid,
            Point {
                y: point.y + 1,
                ..point
            },
            first_cell,
            fill_cell,
        );
    }
    if point.x != 0 {
        fill(
            grid,
            Point {
                x: point.x - 1,
                ..point
            },
            first_cell,
            fill_cell,
        );
    }
    if point.x < grid.size.width - 1 {
        fill(
            grid,
            Point {
                x: point.x + 1,
                ..point
            },
            first_cell,
            fill_cell,
        );
    }
}
