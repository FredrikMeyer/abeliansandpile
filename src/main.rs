mod colors;

use image::{DynamicImage, GenericImage};
use rand::Rng;
use std::env;

use crate::colors::{BLACK, BLUE, GREEN, RED};

fn gen_grid(width: u32, height: u32) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();

    let grid: Vec<Vec<u32>> = (0..width)
        .map(|_| (0..height).map(|_| (rng.gen_range(0..5) as u32)).collect())
        .collect();

    grid
}

fn add_to_grid(grid: &mut Vec<Vec<u32>>, pos_x: usize, pos_y: usize) {
    grid[pos_x][pos_y] = grid[pos_x][pos_y] + 1;
}

fn vertex_is_stable(grid: &Vec<Vec<u32>>, vertex: (usize, usize)) -> bool {
    let val = grid.get(vertex.0).and_then(|r| r.get(vertex.1));

    match val {
        None => panic!("Non existent vertex"),
        Some(v) => return v < &(4 as u32),
    }
}

fn find_unstable_vertex_3(grid: &Vec<Vec<u32>>) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if val >= 4 {
                return Some((y, x));
            }
        }
    }
    return None;
}

fn find_unstable_vertex(grid: &[Vec<u32>]) -> Option<(usize, usize)> {
    let rows = grid.len();
    if rows == 0 {
        return None;
    }

    let cols = grid[0].len();
    let (mid_y, mid_x) = (rows / 2, cols / 2);

    let max_radius = std::cmp::max(mid_y, mid_x);

    for r in 0..=max_radius {
        for y in (mid_y - r)..=(mid_y + r) {
            for x in (mid_x - r)..=(mid_x + r) {
                if x < cols && y < rows {
                    let val = grid[y as usize][x as usize];
                    if val >= 4 {
                        return Some((y, x));
                    }
                }
            }
        }
    }
    None
}

fn find_unstable_vertex_2(grid: &Vec<Vec<u32>>) -> Option<(usize, usize)> {
    let grid_size = grid.len();
    for y in 0..grid_size {
        let row = grid.get(y).unwrap();
        for x in 0..grid_size {
            let val = row.get(x).unwrap();

            if val >= &(4 as u32) {
                return Some((y, x));
            }
        }
    }
    return None;
}

fn topple_vertex(grid: &mut Vec<Vec<u32>>, pos_x: usize, pos_y: usize) {
    let val = grid[pos_x][pos_y];
    let new_val = val - 4;

    if pos_x >= 1 {
        grid[pos_x - 1][pos_y] += 1;
    }
    if pos_x + 1 < grid.len() {
        grid[pos_x + 1][pos_y] += 1;
    }
    if pos_y >= 1 {
        grid[pos_x][pos_y - 1] += 1;
    }
    if pos_y + 1 < grid.len() {
        grid[pos_x][pos_y + 1] += 1;
    }
    grid[pos_x][pos_y] = new_val;
}

fn run_iteration(grid: &mut Vec<Vec<u32>>) {
    let mut prev_unstable: Option<(usize, usize)> = find_unstable_vertex(&grid);

    while let Some((x, y)) = prev_unstable {
        topple_vertex(grid, x, y);

        if vertex_is_stable(grid, (x, y)) {
            prev_unstable = find_unstable_vertex(grid)
        }
    }
}

fn parse_args(args: Vec<String>) -> (usize, u32) {
    println!("Args: {:?}", args);
    let user_n_option = args.get(1);

    let user_n = match user_n_option {
        Some(k) => k,
        None => panic!("Usage example: program 200 1000"),
    };
    let m = user_n.parse::<usize>().unwrap();
    let number_of_sands = args.get(2).unwrap().parse::<u32>().unwrap();

    (m, number_of_sands)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_args = parse_args(args.clone());
    let n = parsed_args.0;
    let number_of_sands = parsed_args.1;

    let mut image = DynamicImage::new_rgb8(n as u32, n as u32);
    let mut grid: Vec<Vec<u32>> = vec![vec![0; n]; n];

    let midpoint = (n / 2) - 1;
    println!("Midpoint {}", midpoint);
    grid[midpoint][midpoint] = number_of_sands; //1000; // 100000;

    run_iteration(&mut grid);

    for (y, row) in grid.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            match &val {
                0 => image.put_pixel(x as u32, y as u32, RED.to_rgba()),
                1 => image.put_pixel(x as u32, y as u32, GREEN.to_rgba()),
                2 => image.put_pixel(x as u32, y as u32, BLUE.to_rgba()),
                3 => image.put_pixel(x as u32, y as u32, BLACK.to_rgba()),
                _ => panic!("Should not happen"),
            }
        }
    }

    image.save("test.png").unwrap();
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{
        add_to_grid, find_unstable_vertex, run_iteration, topple_vertex, vertex_is_stable,
    };

    #[test]
    fn test_vertex_is_stable() {
        let r: Vec<Vec<u32>> = vec![vec![0, 0], vec![0, 0]];

        assert!(vertex_is_stable(&r, (0, 0)));

        let unstable: Vec<Vec<u32>> = vec![vec![0, 5], vec![0, 0]];
        assert!(!vertex_is_stable(&unstable, (0, 1)));
    }

    #[test]
    fn add_1_to_grid() {
        let mut r: Vec<Vec<u32>> = vec![vec![0, 1], vec![0, 0]];

        add_to_grid(&mut r, 0, 1);

        assert_eq!(r[0][1], 2);
    }

    #[test]
    fn test_find_unstable_vertices() {
        let r: Vec<Vec<u32>> = vec![vec![0, 0, 4], vec![5, 0, 0], vec![0, 5, 0]];

        let res = find_unstable_vertex(&r);

        assert_eq!(Some((0, 2)), res);
    }

    #[test]
    fn test_topple_vertex() {
        let mut r: Vec<Vec<u32>> = vec![vec![0, 0, 4], vec![0, 5, 0], vec![0, 5, 0]];

        topple_vertex(&mut r, 1, 1);

        assert_eq!(r[1][1], 1);

        topple_vertex(&mut r, 0, 2);

        assert_eq!(r[0][2], 0);
    }

    #[test]
    fn test_iteration() {
        let mut r: Vec<Vec<u32>> = vec![vec![0, 0, 0], vec![0, 10, 0], vec![0, 0, 0]];

        run_iteration(&mut r);

        assert_eq!(r, vec![vec![0, 2, 0], vec![2, 2, 2], vec![0, 2, 0]]);
    }
}
