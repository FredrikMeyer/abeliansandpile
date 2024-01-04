use std::cell::UnsafeCell;

use crate::point::Point;

pub(crate) trait GridLike {
    fn get(&self, p: Point) -> &u32;
    fn set(&mut self, p: Point, val: u32);
}

pub struct Grid {
    array: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

pub(crate) struct GridIter<'a> {
    grid: &'a Grid,
    current_point: Point,
}

pub(crate) struct GridIterMut<'a> {
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
            let value = UnsafeCell::new(*self.grid.get(self.current_point));
            &mut *value.get()
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
