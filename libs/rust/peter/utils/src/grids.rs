//! Module to handle 2D grids
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Grid {
    data: Vec<u8>,
    w: usize,
    h: usize,
}

impl Grid {
    pub fn data(&self) -> &[u8] {
        self.data.as_ref()
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn to_2d(&self, pos: usize) -> (usize, usize) {
        (pos % self.w, pos / self.w)
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        &self.data[col + self.w * row]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        &mut self.data[col + self.w * row]
    }
}

impl Index<usize> for Grid {
    type Output = u8;

    fn index(&self, pos: usize) -> &Self::Output {
        &self.data[pos]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, pos: usize) -> &mut Self::Output {
        &mut self.data[pos]
    }
}

pub trait MakeGrid {
    fn make_grid(self, boundary: Option<u8>) -> Grid;
}

impl<T: AsRef<[u8]>> MakeGrid for T {
    fn make_grid(self, boundary: Option<u8>) -> Grid {
        let raw_data = self.as_ref();
        let w = raw_data
            .iter()
            .position(|&b| b == b'\n')
            .unwrap_or(raw_data.len());
        let (data, w, h) = boundary
            .map(|boundary| (vec![boundary; w + 2], w + 2, 1))
            .unwrap_or_else(|| (Vec::new(), w, 0));
        let (mut data, mut h) = raw_data
            .split(|&b| b == b'\n')
            .filter(|line| !line.is_empty())
            .fold((data, h), |(mut data, h), line| {
                boundary.map(|boundary| data.push(boundary));
                data.extend_from_slice(line);
                boundary.map(|boundary| data.push(boundary));
                if w * (h + 1) != data.len() {
                    panic!("Inconsistent line length at line {}.", h + 1);
                }
                (data, h + 1)
            });
        boundary.map(|boundary| (h, _) = (h + 1, data.resize(w * (h + 1), boundary)));

        Grid { data, w, h }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_make_grid_with_boundary() {
        let grid = "123\n456\n789\nxyz\n".make_grid(Some(0));
        assert_eq!(5, grid.width());
        assert_eq!(6, grid.height());
        assert_eq!(
            &[
                0, 0, 0, 0, 0, 0, b'1', b'2', b'3', 0, 0, b'4', b'5', b'6', 0, 0, b'7', b'8', b'9',
                0, 0, b'x', b'y', b'z', 0, 0, 0, 0, 0, 0
            ],
            grid.data()
        );
        println!("{:?}", grid);
    }

    #[test]
    pub fn test_make_grid_without_boundary() {
        let grid = "123\n456\n789\nxyz\n".make_grid(None);
        assert_eq!(3, grid.width());
        assert_eq!(4, grid.height());
        assert_eq!(
            &[b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'x', b'y', b'z'],
            grid.data()
        );
        println!("{:?}", grid);
    }

    #[should_panic]
    #[test]
    pub fn test_make_grid_inconsistent() {
        let grid = "123\n456\n789\nvwxyz\n".make_grid(None);
        println!("{:?}", grid);
    }
}
