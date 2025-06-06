mod colors;
mod grid;
pub mod point;

use grid::{Grid, GridLike};
use image::{DynamicImage, GenericImage};
use point::Point;
use rand::Rng;
use rustc_hash::FxHashSet;
use std::{env, fs::File, io::Write};

use crate::colors::{BLACK, BLUE, GREEN, RED};

fn gen_grid(width: u32, height: u32) -> Vec<Vec<u32>> {
    let mut rng = rand::rng();

    let grid: Vec<Vec<u32>> = (0..width)
        .map(|_| {
            (0..height)
                .map(|_| (rng.random_range(0..5) as u32))
                .collect()
        })
        .collect();

    grid
}

fn add_to_grid(grid: &mut Grid, p: Point, amount: u32) {
    grid.set(p, grid.get(p) + amount);
}

/// Find all unstable vertices.
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

fn maybe_topple(p: Point, grid: &mut Grid, unstable_points: &mut FxHashSet<Point>) {
    let max_sands = 4;
    let new_val = grid.get(p.clone()) + 1;
    grid.set(p.clone(), new_val);
    if new_val >= max_sands {
        unstable_points.insert(p);
    }
}

fn topple_vertex(grid: &mut Grid, p: &Point, unstable_points: &mut FxHashSet<Point>) -> bool {
    let val = grid.get(*p);
    let max_sands = 4;
    let new_val = val - max_sands;

    if new_val < 4 {
        unstable_points.remove(&p);
    } else {
        unstable_points.insert(p.clone());
    }

    let pos_x = p.x;
    let pos_y = p.y;

    if pos_x >= 1 {
        let pleft = p.left();
        maybe_topple(pleft, grid, unstable_points);
    }
    if pos_x + 1 < grid.width {
        let pright = p.right();
        maybe_topple(pright, grid, unstable_points);
    }

    if pos_y >= 1 {
        let pdown = p.down();
        maybe_topple(pdown, grid, unstable_points);
    }
    if pos_y + 1 < grid.width {
        let pup = p.up();
        maybe_topple(pup, grid, unstable_points);
    }
    grid.set(*p, new_val);

    new_val >= max_sands
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

fn parse_args(args: Vec<String>) -> (usize, u32, Option<String>, Option<String>) {
    println!("Args: {:?}", args);
    let user_n_option = args.get(1);

    let user_n = match user_n_option {
        Some(k) => k,
        None => panic!("Usage example: program 200 1000 [format]"),
    };
    let m = user_n.parse::<usize>().unwrap();

    let number_of_sands = match args.get(2) {
        Some(n) => n.parse::<u32>().unwrap(),
        None => panic!("Usage example: program 200 1000 [format]"),
    };

    let mirror_option = if args.len() > 3 { Some(args[3].clone()) } else { None };
    let format_option = if args.len() > 4 { Some(args[4].clone()) } else { None };

    (m, number_of_sands, mirror_option, format_option)
}

fn write_to_image(grid: &Grid) {
    let n = grid.width;
    let mut image = DynamicImage::new_rgb8(n as u32, n as u32);

    let as_vec = grid.to_vec();
    for (x, row) in as_vec.iter().enumerate() {
        for (y, &val) in row.iter().enumerate() {
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
    image.save("test.tiff").unwrap();
}

fn mirror_along_diagonal(grid: &Grid) -> Grid {
    let n = grid.size();
    let mut new_grid = Grid::new(n / 2, n / 2);

    for i in 0..(n / 2) {
        for j in ((n / 2) + i)..n {
            let val = grid.get(Point { x: i, y: j });
            new_grid.set(
                Point {
                    x: i,
                    y: j - (n / 2),
                },
                *val,
            );
            new_grid.set(
                Point {
                    x: j - (n / 2),
                    y: i,
                },
                *val,
            );
        }
    }

    new_grid
}

fn write_to_csv(grid: &Grid) {
    let grid_as_string = grid.to_string();

    let res = File::create("output.csv")
        .unwrap()
        .write_all(grid_as_string.as_bytes());

    match res {
        Ok(_) => return,
        Err(_) => println!("Error saving file."),
    }
}

fn write_to_html(grid: &Grid) {
    let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<title>Abelian Sandpile Visualization</title>\n");
    html.push_str("<style>\n");
    html.push_str("table { border-collapse: collapse; }\n");
    html.push_str("td { width: 20px; height: 20px; border: 1px solid #ddd; }\n");
    html.push_str(".color-0 { background-color: #FF0000; } /* RED */\n");
    html.push_str(".color-1 { background-color: #00FF00; } /* GREEN */\n");
    html.push_str(".color-2 { background-color: #0000FF; } /* BLUE */\n");
    html.push_str(".color-3 { background-color: #000000; } /* BLACK */\n");
    html.push_str("</style>\n");
    html.push_str("<script>\n");
    html.push_str("function changeColor(colorClass, newColor) {\n");
    html.push_str("  const elements = document.getElementsByClassName(colorClass);\n");
    html.push_str("  for (let i = 0; i < elements.length; i++) {\n");
    html.push_str("    elements[i].style.backgroundColor = newColor;\n");
    html.push_str("  }\n");
    html.push_str("}\n");
    html.push_str("</script>\n");
    html.push_str("</head>\n<body>\n");

    // Add color pickers
    html.push_str("<div style='margin-bottom: 20px;'>\n");
    html.push_str("  <label>Value 0 (Red): </label>\n");
    html.push_str("  <input type='color' value='#FF0000' onchange='changeColor(\"color-0\", this.value)'><br>\n");
    html.push_str("  <label>Value 1 (Green): </label>\n");
    html.push_str("  <input type='color' value='#00FF00' onchange='changeColor(\"color-1\", this.value)'><br>\n");
    html.push_str("  <label>Value 2 (Blue): </label>\n");
    html.push_str("  <input type='color' value='#0000FF' onchange='changeColor(\"color-2\", this.value)'><br>\n");
    html.push_str("  <label>Value 3 (Black): </label>\n");
    html.push_str("  <input type='color' value='#000000' onchange='changeColor(\"color-3\", this.value)'><br>\n");
    html.push_str("</div>\n");

    // Create table
    html.push_str("<table>\n");

    let grid_data = grid.to_vec();
    for row in grid_data {
        html.push_str("  <tr>\n");
        for cell in row {
            html.push_str(&format!("    <td class='color-{}'></td>\n", cell));
        }
        html.push_str("  </tr>\n");
    }

    html.push_str("</table>\n");
    html.push_str("</body>\n</html>");

    let res = File::create("output.html")
        .unwrap()
        .write_all(html.as_bytes());

    match res {
        Ok(_) => return,
        Err(_) => println!("Error saving HTML file."),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_args = parse_args(args.clone());
    let n = parsed_args.0;
    let number_of_sands = parsed_args.1;
    let mirror_option = parsed_args.2;
    let format_option = parsed_args.3;

    let mut grid: Grid = Grid::new(n, n);
    // let mut grid = Grid::from_vec(gen_grid(n as u32, n as u32));

    let midpoint = Point {
        x: (n / 2) - 1,
        y: (n / 2) - 1,
    };

    add_to_grid(&mut grid, midpoint, number_of_sands);

    // for i in 0..n {
    //     add_to_grid(&mut grid, Point { x: i, y: i }, number_of_sands);
    // }

    // let pt2 = Point {
    //     x: 2 * (n / 3) - 1,
    //     y: (n / 2) - 1,
    // };

    // let pt3 = Point {
    //     x: (n / 2) - 1,
    //     y: (n / 3) - 1,
    // };

    // let pt4 = Point {
    //     x: (n / 2) - 1,
    //     y: 2 * (n / 3) - 1,
    // };
    // println!("Midpoint {}", midpoint);
    // grid.set(pt2, number_of_sands);
    // grid.set(pt4, number_of_sands);
    // grid.set(pt3, number_of_sands);

    run_iteration(&mut grid);

    let output_grid = if mirror_option.is_some() {
        mirror_along_diagonal(&mut grid)
    } else {
        grid
    };

    // Determine which output formats to use
    match format_option.as_deref() {
        Some("csv") => {
            write_to_csv(&output_grid);
            println!("Output written to output.csv");
        },
        Some("html") => {
            write_to_html(&output_grid);
            println!("Output written to output.html");
        },
        Some("png") => {
            write_to_image(&output_grid);
            println!("Output written to test.png");
        },
        Some("all") => {
            write_to_csv(&output_grid);
            write_to_html(&output_grid);
            write_to_image(&output_grid);
            println!("Output written to output.csv, output.html, and test.png");
        },
        _ => {
            // Default: output to all formats
            write_to_csv(&output_grid);
            write_to_html(&output_grid);
            write_to_image(&output_grid);
            println!("Output written to output.csv, output.html, and test.png");
        }
    }
}

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashSet;

    use crate::{
        add_to_grid, find_unstable_vertex, find_unstable_vertices, run_iteration, topple_vertex,
        Grid, GridLike, Point,
    };

    #[test]
    fn add_1_to_grid() {
        let r: Vec<Vec<u32>> = vec![vec![0, 1], vec![0, 0]];
        let mut g = Grid::from_vec(r);

        add_to_grid(&mut g, Point { x: 0, y: 1 }, 1);

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
