mod colors;
pub mod point;

use image::{DynamicImage, GenericImage};
use point::Point;
use rand::Rng;
use rustc_hash::FxHashSet;
use std::env;

use crate::colors::{BLACK, BLUE, GREEN, RED};

trait GridLike {
    fn get(&self, p: Point) -> &u32;
    fn set(&mut self, p: Point, val: u32);
}

struct Grid {
    array: Vec<u32>,
    width: usize,
    height: usize,
}

struct GridIter<'a> {
    grid: &'a Grid,
    current_point: Point,
}

struct GridIterMut<'a> {
    grid: &'a mut Grid,
    current_point: Point,
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

    pub fn iter(&self) -> GridIter {
        GridIter {
            grid: self,
            current_point: Point { x: 0, y: 0 },
        }
    }

    pub fn iter_mut(&mut self) -> GridIterMut {
        GridIterMut {
            grid: self,
            current_point: Point { x: 0, y: 0 },
        }
    }
}

impl<'a> Iterator for GridIter<'a> {
    type Item = (Point, &'a u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_point.x >= self.grid.width {
            self.current_point.x = 0;
            self.current_point.y += 1;
        }

        if self.current_point.y >= self.grid.height {
            return None;
        }

        let value = self.grid.get(self.current_point);
        let result = (self.current_point, value);

        self.current_point.x += 1;

        Some(result)
    }
}

impl<'a> Iterator for GridIterMut<'a> {
    type Item = (Point, &'a mut u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_point.x >= self.grid.width {
            self.current_point.x = 0;
            self.current_point.y += 1;
        }

        if self.current_point.y >= self.grid.height {
            return None;
        }

        let value = unsafe {
            // This is safe because we're ensuring only one mutable reference exists at a time
            let ptr = self.grid.get(self.current_point) as *const u32 as *mut u32;
            &mut *ptr
        };
        let result = (self.current_point, value);

        self.current_point.x += 1;

        Some(result)
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

fn find_unstable_vertices(grid: &Grid) -> FxHashSet<Point> {
    let mut points = FxHashSet::default();

    for (point, value) in grid.iter() {
        if value >= &(4 as u32) {
            points.insert(point);
        }
    }
    points
}

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

fn topple_vertex(grid: &mut Grid, p: &Point, unstable_points: &mut FxHashSet<Point>) -> bool {
    let val = grid.get(*p);
    let new_val = val - 4;

    if new_val < 4 {
        unstable_points.remove(&p);
    } else {
        unstable_points.insert(p.clone());
    }

    let pos_x = p.x;
    let pos_y = p.y;

    if pos_x >= 1 {
        let pleft = p.left();
        let new_val = grid.get(pleft) + 1;
        grid.set(pleft, new_val);
        if new_val >= 4 {
            unstable_points.insert(pleft);
        }
    }
    if pos_x + 1 < grid.width {
        let pright = p.right();
        let new_val = grid.get(pright) + 1;
        grid.set(pright, new_val);
        if new_val >= 4 {
            unstable_points.insert(pright);
        }
    }
    if pos_y >= 1 {
        let pdown = p.down();
        let new_val = grid.get(pdown) + 1;
        grid.set(pdown, new_val);
        if new_val >= 4 {
            unstable_points.insert(pdown);
        }
    }
    if pos_y + 1 < grid.width {
        let pup = p.up();
        let new_val = grid.get(pup) + 1;
        grid.set(pup, new_val);
        if new_val >= 4 {
            unstable_points.insert(pup);
        }
    }
    grid.set(*p, new_val);

    new_val >= 4
}

fn run_iteration(grid: &mut Grid) {
    let mut all_unstables = find_unstable_vertices(&grid);

    while let Some(p) = all_unstables
        .iter()
        .next()
        .cloned()
        .and_then(|p| all_unstables.take(&p))
    {
        topple_vertex(grid, &p, &mut all_unstables);
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

    // let pt2 = Point {
    //     x: 2 * (n / 3) - 1,
    //     y: (n / 2) - 1,
    // };
    // println!("Midpoint {}", midpoint);
    // grid.set(pt2, number_of_sands);
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
}

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashSet;

    use crate::{
        add_to_grid, find_unstable_vertex, find_unstable_vertices, run_iteration, topple_vertex,
        vertex_is_stable, Grid, GridLike, Point,
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
        let r: Vec<Vec<u32>> = vec![vec![0, 1], vec![0, 0]];
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
        let r: Vec<Vec<u32>> = vec![vec![0, 0, 4], vec![0, 5, 0], vec![0, 5, 0]];
        let mut g = Grid::from_vec(r);
        let p = Point { x: 1, y: 1 };
        let mut unstables = FxHashSet::default();
        unstables.insert(Point { x: 0, y: 2 });

        topple_vertex(&mut g, &p, &mut unstables);

        let res = g.to_vec();
        assert_eq!(res[1][1], 1);

        topple_vertex(&mut g, &Point { x: 0, y: 2 }, &mut unstables);

        let res2 = g.to_vec();
        assert_eq!(res2[0][2], 0);
    }

    #[test]
    fn test_iteration() {
        let r: Vec<Vec<u32>> = vec![vec![0, 0, 0], vec![0, 10, 0], vec![0, 0, 0]];
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

    #[test]
    fn test_find_unstable_vertices_set() {
        let mut g = Grid::new(10, 10);
        g.set(Point { x: 0, y: 0 }, 1);
        g.set(Point { x: 5, y: 5 }, 2);
        g.set(Point { x: 7, y: 5 }, 3);
        g.set(Point { x: 5, y: 7 }, 4);

        let mut res = find_unstable_vertices(&g);

        assert_eq!(res.len(), 1);
        let gg = res.iter().next().cloned().and_then(|p| res.take(&p));
        assert_eq!(gg.unwrap(), Point { x: 5, y: 7 });
        assert_eq!(res.len(), 0);
    }
}
