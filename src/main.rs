mod colors;

use image::{DynamicImage, GenericImage};
use rand::Rng;
use std::env;

use crate::colors::{BLACK, BLUE, GREEN, RED};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
}

trait GridLike {
    fn get(&self, p: Point) -> &u32;
    fn set(&mut self, p: Point, val: u32);
}

struct Grid {
    array: Vec<u32>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            array: [0].repeat(width * height),
            width,
            height,
        }
    }

    pub fn from_vec(vec: Vec<Vec<u32>>) -> Grid {
        let height = vec.len();
        let width = vec[0].len();

        let array: Vec<u32> = vec.into_iter().flatten().collect();

        Grid {
            array,
            width,
            height,
        }
    }

    pub fn to_vec(&self) -> Vec<Vec<u32>> {
        let mut result = Vec::with_capacity(self.height);

        for i in 0..self.height {
            let start_index = i * self.width;
            let end_index = start_index + self.width;
            result.push(self.array[start_index..end_index].to_vec());
        }

        result
    }
}

impl GridLike for Grid {
    fn get(&self, p: Point) -> &u32 {
        self.array
            .get(p.x * self.width + p.y)
            .unwrap_or(&(0 as u32))
    }

    fn set(&mut self, p: Point, val: u32) {
        self.array[p.x * self.width + p.y] = val
    }
}

fn gen_grid(width: u32, height: u32) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();

    let grid: Vec<Vec<u32>> = (0..width)
        .map(|_| (0..height).map(|_| (rng.gen_range(0..5) as u32)).collect())
        .collect();

    grid
}

fn add_to_grid(grid: &mut Grid, p: Point) {
    grid.set(p, grid.get(p) + 1);
}

fn vertex_is_stable(grid: &Grid, vertex: Point) -> bool {
    let val = grid.get(vertex);
    return val < &(4 as u32);
}

// fn find_unstable_vertex_3(grid: &Grid) -> Option<Point> {
//     for (y, row) in grid.iter().enumerate() {
//         for (x, &val) in row.iter().enumerate() {
//             if val >= 4 {
//                 return Some((y, x));
//             }
//         }
//     }
//     return None;
// }

fn find_unstable_vertex(grid: &Grid) -> Option<Point> {
    let rows = grid.width;
    if rows == 0 {
        return None;
    }

    let cols = grid.height;
    let (mid_y, mid_x) = (rows / 2, cols / 2);

    let max_radius = std::cmp::max(mid_y, mid_x);

    for r in 0..=max_radius {
        for y in (mid_y - r)..=(mid_y + r) {
            for x in (mid_x - r)..=(mid_x + r) {
                if x < cols && y < rows {
                    let val = grid.get(Point { x, y });
                    if val >= &4 {
                        return Some(Point { x, y });
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

fn topple_vertex(grid: &mut Grid, p: Point) -> bool {
    let val = grid.get(p);
    let new_val = val - 4;

    let pos_x = p.x;
    let pos_y = p.y;

    if pos_x >= 1 {
        let pleft = p.left();
        grid.set(pleft, grid.get(pleft) + 1);
    }
    if pos_x + 1 < grid.width {
        grid.set(p.right(), grid.get(p.right()) + 1);
    }
    if pos_y >= 1 {
        grid.set(p.down(), grid.get(p.down()) + 1);
    }
    if pos_y + 1 < grid.width {
        grid.set(p.up(), grid.get(p.up()) + 1);
    }
    grid.set(p, new_val);

    new_val >= 4
}

fn run_iteration(grid: &mut Grid) {
    let mut prev_unstable: Option<Point> = find_unstable_vertex(&grid);

    while let Some(p) = prev_unstable {
        let still_unstable = topple_vertex(grid, p);

        if !still_unstable {
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
    let mut grid: Grid = Grid::new(n, n);

    let midpoint = Point {
        x: (n / 2) - 1,
        y: (n / 2) - 1,
    };
    // println!("Midpoint {}", midpoint);
    grid.set(midpoint, number_of_sands);

    run_iteration(&mut grid);

    let as_vec = grid.to_vec();
    for (y, row) in as_vec.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            // println!("{}, {}", x, y);
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
        add_to_grid, find_unstable_vertex, run_iteration, topple_vertex, vertex_is_stable, Grid,
        GridLike, Point,
    };

    #[test]
    fn test_vertex_is_stable() {
        let r: Vec<Vec<u32>> = vec![vec![0, 0], vec![0, 0]];

        let g = Grid::from_vec(r);

        assert!(vertex_is_stable(&g, Point { x: 0, y: 0 }));

        let unstable: Vec<Vec<u32>> = vec![vec![0, 5], vec![0, 0]];
        let unstable_grid = Grid::from_vec(unstable);
        assert!(!vertex_is_stable(&unstable_grid, Point { x: 0, y: 1 }));
    }

    #[test]
    fn add_1_to_grid() {
        let mut r: Vec<Vec<u32>> = vec![vec![0, 1], vec![0, 0]];
        let mut g = Grid::from_vec(r);

        add_to_grid(&mut g, Point { x: 0, y: 1 });

        let res = g.to_vec();
        assert_eq!(res[0][1], 2);
    }

    #[test]
    fn test_find_unstable_vertices() {
        let r: Vec<Vec<u32>> = vec![vec![0, 0, 4], vec![2, 0, 0], vec![0, 1, 0]];
        let g = Grid::from_vec(r);

        let res = find_unstable_vertex(&g);

        assert_eq!(Some(Point { x: 0, y: 2 }), res);
    }

    #[test]
    fn test_topple_vertex() {
        let mut r: Vec<Vec<u32>> = vec![vec![0, 0, 4], vec![0, 5, 0], vec![0, 5, 0]];
        let mut g = Grid::from_vec(r);
        let p = Point { x: 1, y: 1 };
        topple_vertex(&mut g, p);

        let res = g.to_vec();
        assert_eq!(res[1][1], 1);

        topple_vertex(&mut g, Point { x: 0, y: 2 });

        let res2 = g.to_vec();
        assert_eq!(res2[0][2], 0);
    }

    #[test]
    fn test_iteration() {
        let mut r: Vec<Vec<u32>> = vec![vec![0, 0, 0], vec![0, 10, 0], vec![0, 0, 0]];
        let mut g = Grid::from_vec(r);

        run_iteration(&mut g);

        let res = g.to_vec();

        assert_eq!(res, vec![vec![0, 2, 0], vec![2, 2, 2], vec![0, 2, 0]]);
    }

    #[test]
    fn test_grid_impl() {
        let mut g = Grid::new(10, 10);

        g.set(Point { x: 0, y: 0 }, 1);
        g.set(Point { x: 5, y: 5 }, 2);
        g.set(Point { x: 7, y: 5 }, 3);
        g.set(Point { x: 5, y: 7 }, 4);

        let getted = g.get(Point { x: 0, y: 0 });
        let getted_2 = g.get(Point { x: 5, y: 5 });
        let getted_3 = g.get(Point { x: 7, y: 5 });
        let getted_4 = g.get(Point { x: 5, y: 7 });

        assert_eq!(getted, &1);
        assert_eq!(getted_2, &2);
        assert_eq!(getted_3, &3);
        assert_eq!(getted_4, &4);
    }
}
