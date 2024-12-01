#[derive(Clone)]
pub struct Grid<T> {
    entries: Vec<Vec<T>>,
    width: isize,
    height: isize,
    wrapping: bool,
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.entries {
            for x in row {
                write!(f, "{}", x)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn new<F>(width: usize, height: usize, mut init: F) -> Self
    where
        F: FnMut(isize, isize) -> T,
    {
        let width = width as isize;
        let height = height as isize;
        let entries =
            Vec::from_iter((0..height).map(|y| Vec::from_iter((0..width).map(|x| init(x, y)))));
        Self {
            entries,
            width,
            height,
            wrapping: false,
        }
    }

    pub fn set_wrapping(&mut self, wrapping: bool) {
        self.wrapping = wrapping;
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.height
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            None
        } else {
            unsafe {
                Some(
                    self.entries
                        .get_unchecked(y as usize)
                        .get_unchecked(x as usize),
                )
            }
        }
    }

    pub fn get_wrapped(&self, x: isize, y: isize) -> &T {
        let x = x.rem_euclid(self.width);
        let y = y.rem_euclid(self.height);

        unsafe {
            self.entries
                .get_unchecked(y as usize)
                .get_unchecked(x as usize)
        }
    }

    pub fn get_wrapped_mut(&mut self, x: isize, y: isize) -> &mut T {
        let x = x.rem_euclid(self.width);
        let y = y.rem_euclid(self.height);

        unsafe {
            self.entries
                .get_unchecked_mut(y as usize)
                .get_unchecked_mut(x as usize)
        }
    }

    pub fn parse<F>(content: &str, mut f: F) -> Self
    where
        F: FnMut(char) -> T,
    {
        let entries = content
            .lines()
            .map(|line| line.chars().map(&mut f).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self::from(entries)
    }

    pub fn parse_with_default<F, G>(content: &str, mut f: F, mut default: G) -> Self
    where
        F: FnMut(char) -> T,
        G: FnMut() -> T,
    {
        let mut entries = content
            .lines()
            .map(|line| line.chars().map(&mut f).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = entries
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or_default();

        for row in &mut entries {
            for _ in row.len()..width {
                row.push(default())
            }
        }

        Self::from(entries)
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(entries: Vec<Vec<T>>) -> Self {
        let height = entries.len() as isize;
        let width = entries.get(0).map(|row| row.len()).unwrap_or_default() as isize;

        for row in &entries {
            assert_eq!(row.len(), width as usize);
        }

        Self {
            entries,
            width,
            height,
            wrapping: false,
        }
    }
}

impl<T> std::ops::Index<(isize, isize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        if self.wrapping {
            self.get_wrapped(x, y)
        } else {
            self.get(x, y).unwrap()
        }
    }
}

impl<T> std::ops::IndexMut<(isize, isize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        if self.wrapping {
            self.get_wrapped_mut(x, y)
        } else {
            &mut self.entries[y as usize][x as usize]
        }
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self[(x as isize, y as isize)]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self[(x as isize, y as isize)]
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = ((isize, isize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.grid.width() as usize {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= self.grid.height() as usize {
            return None;
        }

        let res = (
            (self.x as isize, self.y as isize),
            &self.grid[(self.x, self.y)],
        );

        self.x += 1;

        Some(res)
    }
}

impl<'a, T> std::iter::IntoIterator for &'a Grid<T> {
    type Item = ((isize, isize), &'a T);
    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

const GRID_NEIGHBOR_ORDER: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub struct GridNeighborsIter<'a, T> {
    grid: &'a Grid<T>,
    x: isize,
    y: isize,
    current: usize,
    relevant: u8,
}

impl<'a, T> Iterator for GridNeighborsIter<'a, T> {
    type Item = ((isize, isize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.current < 8 && (self.relevant >> self.current) & 1 == 0 {
            self.current += 1;
        }

        if self.current >= 8 {
            return None;
        }

        let x = self.x + GRID_NEIGHBOR_ORDER[self.current].0;
        let y = self.y + GRID_NEIGHBOR_ORDER[self.current].1;

        let res = Some(((x, y), &self.grid[(x, y)]));
        self.current += 1;
        res
    }
}

impl<T> Grid<T> {
    pub fn neighbors8(&self, x: isize, y: isize) -> GridNeighborsIter<T> {
        let mut relevant = 0xFF;

        if !self.wrapping {
            for i in 0..8 {
                let offset = GRID_NEIGHBOR_ORDER[i];
                let x = x + offset.0;
                let y = y + offset.1;

                if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
                    relevant = relevant ^ 1 << i;
                }
            }
        }

        GridNeighborsIter {
            grid: self,
            x,
            y,
            current: 0,
            relevant,
        }
    }

    pub fn neighbors4(&self, x: isize, y: isize) -> GridNeighborsIter<T> {
        let mut relevant = 0xFF;

        for i in 0..8 {
            let offset = GRID_NEIGHBOR_ORDER[i];

            if offset.0.abs() + offset.1.abs() == 2 {
                relevant = relevant ^ 1 << i;
                continue;
            }

            let x = x + offset.0;
            let y = y + offset.1;

            if !self.wrapping {
                if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
                    relevant = relevant ^ 1 << i;
                }
            }
        }

        GridNeighborsIter {
            grid: self,
            x,
            y,
            current: 0,
            relevant,
        }
    }
}
