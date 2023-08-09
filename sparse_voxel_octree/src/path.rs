#[inline]
pub fn idx_is_valid(idx: [usize; 3], depth: u8) -> bool {
    !idx.iter().any(|val| *val >> depth > 0)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Octant {
    LeftLowerFront,
    RightLowerFront,
    LeftUpperFront,
    RightUpperFront,
    LeftLowerBack,
    RightLowerBack,
    LeftUpperBack,
    RightUpperBack,
}

impl Octant {
    #[inline]
    pub fn new(val: usize) -> Self {
        match val {
            0 => Octant::LeftLowerFront,
            1 => Octant::RightLowerFront,
            2 => Octant::LeftUpperFront,
            3 => Octant::RightUpperFront,
            4 => Octant::LeftLowerBack,
            5 => Octant::RightLowerBack,
            6 => Octant::LeftUpperBack,
            7 => Octant::RightUpperBack,
            _ => panic!(),
        }
    }

    #[inline]
    pub fn val(&self) -> usize {
        match self {
            Octant::LeftLowerFront => 0,
            Octant::RightLowerFront => 1,
            Octant::LeftUpperFront => 2,
            Octant::RightUpperFront => 3,
            Octant::LeftLowerBack => 4,
            Octant::RightLowerBack => 5,
            Octant::LeftUpperBack => 6,
            Octant::RightUpperBack => 7,
        }
    }
}

#[inline]
pub fn path_to(pos: &[usize; 3], depth: u8) -> Vec<Octant> {
    let mut result: Vec<Octant> = Vec::with_capacity(depth.into());
    let mut mask = if depth != 0 { 0b01 << (depth - 1) } else { 0 };
    while mask > 0 {
        result.push(Octant::new(
            (pos[2] & mask) >> mask.trailing_zeros() << 2
                | (pos[1] & mask) >> mask.trailing_zeros() << 1
                | (pos[0] & mask) >> mask.trailing_zeros(),
        ));
        mask >>= 1;
    }
    result
}

pub struct Path {
    data: Box<[Octant]>,
    pos: usize,
}
impl Path {
    pub fn new(pos: [usize; 3], depth: u8) -> Self {
        Self {
            data: path_to(&pos, depth).into_boxed_slice(),
            pos: 0,
        }
    }

    #[inline]
    pub fn is_done(&self) -> bool {
        self.pos >= self.data.len()
    }

    #[inline]
    pub fn walk(&mut self) -> Option<Octant> {
        match self.data.get(self.pos) {
            Some(val) => {
                self.pos += 1;
                Some(*val)
            }
            None => None,
        }
    }

    #[inline]
    pub fn step_up(&mut self) {
        self.pos -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_to_path_test_one() {
        let test_data = &[
            ([0, 0, 0], Octant::LeftLowerFront),
            ([1, 0, 0], Octant::RightLowerFront),
            ([0, 1, 0], Octant::LeftUpperFront),
            ([1, 1, 0], Octant::RightUpperFront),
            ([0, 0, 1], Octant::LeftLowerBack),
            ([1, 0, 1], Octant::RightLowerBack),
            ([0, 1, 1], Octant::LeftUpperBack),
            ([1, 1, 1], Octant::RightUpperBack),
        ];
        for (pos, idx) in test_data {
            assert_eq!(path_to(pos, 1)[0], *idx)
        }
    }

    #[test]
    fn pos_to_path_test_two() {
        let test_data = &[
            ([0, 0, 0], [Octant::LeftLowerFront, Octant::LeftLowerFront]),
            ([1, 1, 1], [Octant::LeftLowerFront, Octant::RightUpperBack]),
            ([2, 2, 2], [Octant::RightUpperBack, Octant::LeftLowerFront]),
            ([3, 3, 3], [Octant::RightUpperBack, Octant::RightUpperBack]),
        ];
        for (pos, idx) in test_data {
            assert_eq!(path_to(pos, 2).as_slice(), idx)
        }
    }
}
