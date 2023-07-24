#[derive(Clone, Copy)]
pub struct Point(pub isize, pub isize);

impl Point {
    pub fn x(&self) -> isize {
        self.1
    }

    pub fn y(&self) -> isize {
        self.0
    }

    pub fn set_x(&mut self, x: isize) {
        self.1 = x;
    }

    pub fn set_y(&mut self, y: isize) {
        self.0 = y;
    }
}

impl core::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl core::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl core::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1
    }
}

impl core::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

#[derive(Clone, Copy)]
pub struct Angle(usize);

impl Angle {
    // Creates a new angle guaranteed to have a valid degree value in the system
    pub fn new(n: usize) -> Self {
        // Should be wrapped around
        let n = n % 360;
        // And rounded to the nearest defined angle
        let n = n - (n % 15);
        Self(n)
    }

    pub fn reflect(self, side_collided: SideCollided) -> Self {
        match self.0 {
            // Edge cases
            0 => Self(180),
            90 => Self(270),
            180 => Self(0),
            270 => Self(90),
            _ => match side_collided {
                // Collision with the verical side of an object
                SideCollided::Vertical => match self.0 {
                    0..=180 => Self(180 - self.0),
                    _ => Self(540 - self.0)
                },
                // Collision with the horizontal side of an object
                SideCollided::Horizontal => Self(360 - self.0)
            }
        }
    }

    pub fn sin(&self) -> isize {
        match self.0 {
            0 => 0,
            15 => -1,
            30 => -1,
            45 => -1,
            60 => -2,
            75 => -3,
            90 => -1,
            105 => -3,
            120 => -2,
            135 => -1,
            150 => -1,
            165 => -1,
            180 => -1,
            195 => 1,
            210 => 1,
            225 => 1,
            240 => 2,
            255 => 3,
            270 => 1,
            285 => 3,
            300 => 2,
            315 => 1,
            330 => 1,
            345 => 1,
            // The number self.0 is guaranteed to always be a valid degree
            _ => unreachable!()
        }
    }

    pub fn cos(&self) -> isize {
        match self.0 {
            0 => 1,
            15 => 3,
            30 => 2,
            45 => 1,
            60 => 1,
            75 => 1,
            90 => 0,
            105 => -1,
            120 => -1,
            135 => -1,
            150 => -2,
            165 => -3,
            180 => 0,
            195 => -3,
            210 => -2,
            225 => -1,
            240 => -1,
            255 => -1,
            270 => 0,
            285 => 1,
            300 => 1,
            315 => 1,
            330 => 2,
            345 => 3,
            // The number self.0 is guaranteed to always be a valid degree
            _ => unreachable!()
        }
    }
}

pub enum SideCollided {
    Vertical,
    Horizontal
}
