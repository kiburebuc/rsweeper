use num_traits::Num;
use num_traits::cast::AsPrimitive;

#[derive(Clone, PartialEq, Default, Copy, Debug)]
pub struct Coord<T: Num = usize>(pub T, pub T);

impl<T: Num + Copy> Coord<T> {
    pub fn x(&self) -> T { self.0 }
    pub fn y(&self) -> T { self.1 }
    pub fn w(&self) -> T { self.0 }
    pub fn h(&self) -> T { self.1 }
    pub fn mul(&self) -> T { self.0 * self.1 }
}

impl<T: Copy + Num + 'static> Coord<T> {
    pub fn from<F: AsPrimitive<T> + Num>(co: Coord<F>) -> Coord<T> {
        Coord(co.0.as_(), co.1.as_())
    }
}

#[derive(Default, Clone)]
pub struct Array2d<T: Default + Clone + PartialEq> {
    data: Vec<T>,
    size: Coord,
}

impl<T: Default + Clone + PartialEq> Array2d<T> {
    pub fn resize(&mut self, size: Coord) {
        self.data.resize_with(size.mul(), Default::default);
        self.size = size;
    }

    pub fn access(&self, co: Coord) -> &T {
        &self.data[(co.x() * self.size.w() + co.y()) as usize]
    }

    pub fn safe_access(&self, co: Coord<isize>) -> Option<&T> {
        if self.in_bounds(co) {
            Some(self.access(Coord::from(co)))
        } else { None }
    }

    pub fn in_bounds(&self, co: Coord<isize>) -> bool {
        co.x() >= 0 && co.y() >= 0 && 
            co.x() < self.size.w() as isize && co.y() < self.size.h() as isize
    }

    pub fn access_mut(&mut self, co: Coord<usize>) -> &mut T {
        &mut self.data[(co.x() * self.size.w() + co.y()) as usize]
    }
    
    pub fn get_row(&self, y: usize) -> &[T] {
        &self.data[(self.size.w() * y)..(self.size.w() * (y + 1))]
    }

    pub fn len(&self) -> usize { self.data.len() }

    pub fn enumerate_mut(&mut self) -> Array2dIter<T> {
        Array2dIter {
            size: self.size,
            data: self.data.iter_mut(),
            co: Coord(0, 0),
        }
    }

    pub fn get_rows(&self) -> Array2dRowIter<T> {
        Array2dRowIter {
            data: self,
            y: 0,
            h: self.size.h(),
        }
    }

    pub fn width(&self) -> usize { self.size.w() }
    pub fn height(&self) -> usize { self.size.h() }

    pub fn get_surround(&self, co: Coord<isize>) -> Vec<Option<&T>> {
        let mut result = Vec::<Option<&T>>::default();
        
        for xoff in -1..2 {
            for yoff in -1..2 {
                result.push(self.safe_access(Coord(co.x() + xoff, co.y() + yoff)));
            }
        }

        result
    }

    pub fn get_surround_match(&self, co: Coord<isize>, f: fn(&T) -> bool) -> u8 {
        let mut result = 0;
        let surr = self.get_surround(co);
        for cell in surr {
            if let Some(c) = cell {
                if f(c) {
                    result += 1;
                }
            }
        }
        result
    }
}

// Iterators vvvvvv
pub struct Array2dIter<'a, T: Default + Clone + PartialEq + 'a> {
    data: std::slice::IterMut<'a, T>,
    co: Coord<usize>,
    size: Coord<usize>,
}

pub struct Array2dRowIter<'a, T: Default + Clone + PartialEq> {
    data: &'a Array2d<T>,
    y: usize,
    h: usize,
}

impl<'a, T: Default + Clone + PartialEq + 'a> Iterator for Array2dIter<'a, T> {
    type Item = (Coord, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.co.y() >= self.size.h() { // if wrapped around, goto next row
            self.co.1 = 0;
            self.co.0 += 1;
        }
        let Some(result) = self.data.next() else { return None; }; 
        let result = (self.co, result);
        self.co.1 += 1;
        Some(result)
    }
}

impl<'a, T: Default + Clone + PartialEq> Iterator for Array2dRowIter<'a, T> {
    type Item = (usize, &'a [T]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.h {
            None
        } else {
            let result = (self.y, self.data.get_row(self.y));
            self.y += 1;
            Some(result)
        }
    }
}
