use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rotation(pub [[isize; 2]; 2]);

impl Rotation {
    pub const fn identity() -> Self {
        Self([
            [1, 0],
            [0, 1],
        ])
    }

    pub fn inverse(self) -> Self {
        Self([
            [self.0[1][1], -self.0[0][1]],
            [-self.0[1][0], self.0[0][0]],
        ])
    }

    pub fn rotate_90deg_right(self) -> Self {
        Rotation([
            [0, -1],
            [1, 0],
        ]) * self
    }
}

impl Mul<[isize; 2]> for Rotation {
    type Output = [isize; 2];

    fn mul(self, rhs: [isize; 2]) -> Self::Output {
        [
            rhs[0] * self.0[0][0] + rhs[1] * self.0[0][1],
            rhs[0] * self.0[1][0] + rhs[1] * self.0[1][1],
        ]
    }
}

impl Mul<Rotation> for Rotation {
    type Output = Self;

    fn mul(self, rhs: Rotation) -> Self::Output {
        Self([
            [
                rhs.0[0][0] * self.0[0][0] + rhs.0[1][0] * self.0[0][1],
                rhs.0[0][1] * self.0[0][0] + rhs.0[1][1] * self.0[0][1],
            ],
            [
                rhs.0[0][0] * self.0[1][0] + rhs.0[1][0] * self.0[1][1],
                rhs.0[0][1] * self.0[1][0] + rhs.0[1][1] * self.0[1][1],
            ],
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_inverse() {
        let id = Rotation([
            [1, 0],
            [0, 1],
        ]);
        assert_eq!(id.inverse(), id);

        let rot_right = Rotation([
            [0, -1],
            [1, 0],
        ]);
        let rot_left = Rotation([
            [0, 1],
            [-1, 0],
        ]);
        assert_eq!(rot_right.inverse(), rot_left);
        assert_eq!(rot_left.inverse(), rot_right);
    }

    #[test]
    fn test_mul_vec() {
        let m = Rotation([
            [1, 2],
            [3, 4],
        ]);
        assert_eq!(m * [3, 4], [11, 25]);
    }

    #[test]
    fn test_mul_mat() {
        let m1 = Rotation([
            [3, 4],
            [5, 6],
        ]);
        let m2 = Rotation([
            [1, 2],
            [3, 4],
        ]);
        assert_eq!(m1 * m2, Rotation([
            [15, 22],
            [23, 34],
        ]));
    }

    #[test]
    fn test_rotate_90deg_right() {
        let mut m = Rotation::identity();

        m = m.rotate_90deg_right();
        assert_eq!(m, Rotation([
            [0, -1],
            [1, 0],
        ]));

        m = m.rotate_90deg_right();
        assert_eq!(m, Rotation([
            [-1, 0],
            [0, -1],
        ]));

        m = m.rotate_90deg_right();
        assert_eq!(m, Rotation([
            [0, 1],
            [-1, 0],
        ]));

        m = m.rotate_90deg_right();
        assert_eq!(m, Rotation::identity());
    }
}
