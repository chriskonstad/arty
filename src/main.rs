#[derive(Debug, Clone, Copy)]
struct Distance(f64);

#[derive(Debug, Clone, Copy)]
struct Mils(f64);

#[derive(Debug, Clone, Copy)]
struct Degrees(f64);

/**
 * TODO
 * UX
 * Parsing
 */

/*
 * NOTES:
 * Traverse goes: - ==== + (negative to the left, positive to the right)
 * This assumes the standard
 *   A B C               +---> x+
 * 1             N       |
 * 2           W   E     V
 * 3             S       y+
 * mapping for coordinates.
 */

#[derive(Debug)]
struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn dist(&self, other: &Pos) -> Distance {
        Distance(((self.x - other.x).abs().powi(2) + (self.y - other.y).abs().powi(2)).sqrt())
    }

    // TODO Move this onto the gun struct's impl
    pub fn angle(&self, other: &Pos, zero: Degrees) -> Degrees {
        // Align the compass's 0 with the mathematical zero.
        let theta_rad = (zero.0 - 90.0) * std::f64::consts::PI / 180.0;

        // Translate so the gun is at zero
        let x_z = other.x - self.x;
        let y_z = other.y - self.y;

        // Rotate so we zero on the gun's zero
        let x_r = x_z * theta_rad.cos() + y_z * theta_rad.sin();
        let y_r = -x_z * theta_rad.sin() + y_z * theta_rad.cos();

        Degrees(y_r.atan2(x_r) * 180.0 / std::f64::consts::PI)
    }
}

#[derive(Debug)]
struct Gun {
    pub pos: Pos,
    pub heading_at_zero: Degrees,
}

#[derive(Debug)]
struct Target {
    pub pos: Pos,
}

fn dist_to_mils(distance: Distance) -> Mils {
    Mils(-0.2371 * distance.0 + 1001.5)
}

fn calc(gun: Gun, target: Target) -> (Mils, Degrees) {
    (dist_to_mils(gun.pos.dist(&target.pos)), gun.pos.angle(&target.pos, gun.heading_at_zero))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dist() {
        let p0 = Pos {
            x: 1.0,
            y: 2.0,
        };
        let p1 = Pos {
            x: 4.0,
            y: 6.0,
        };
        assert_eq!(5.0, p0.dist(&p1).0);
        assert_eq!(5.0, p1.dist(&p0).0);
    }

    #[test]
    fn test_dist_to_mils_conversion() {
        assert_eq!(866.0, dist_to_mils(Distance(572.0)).0.round());
    }

    #[test]
    fn test_traverse_north() {
        let tester = |zero, expected| {
            let g = Gun {
                pos: Pos {
                    x: 5.0,
                    y: 10.0,
                },
                heading_at_zero: Degrees(zero),
            };
            let t = Target {
                pos: Pos {
                    x: 8.0,
                    y: 2.0,
                },
            };
            let (_, trav) = calc(g, t);
            assert_eq!(expected, trav.0.round());
        };
        tester(0.0, 21.0);
        tester(270.0, 90.0 + 21.0);
        tester(90.0, 21.0 - 90.0);
        tester(300.0, 60.0 + 21.0);
        tester(45.0, 21.0 - 45.0);
    }

    #[test]
    fn test_traverse_south() {
        let tester = |zero, expected| {
            let g = Gun {
                pos: Pos {
                    x: 5.0,
                    y: 0.0,
                },
                heading_at_zero: Degrees(zero),
            };
            let t = Target {
                pos: Pos {
                    x: 8.0,
                    y: 8.0,
                },
            };
            let (_, trav) = calc(g, t);
            assert_eq!(expected, trav.0.round());
        };
        tester(180.0, -(21.0));
        tester(270.0, -(21.0 + 90.0));
        tester(90.0, -(21.0 - 90.0));
        tester(240.0, -(21.0 + 60.0));
        tester(135.0, -(21.0 - 45.0));
    }

    #[test]
    fn test_traverse_east() {
        let tester = |zero, expected| {
            let g = Gun {
                pos: Pos {
                    x: 0.0,
                    y: 5.0,
                },
                heading_at_zero: Degrees(zero),
            };
            let t = Target {
                pos: Pos {
                    x: 8.0,
                    y: 2.0,
                },
            };
            let (_, trav) = calc(g, t);
            assert_eq!(expected, trav.0.round());
        };
        tester(90.0, -21.0);
        tester(0.0, 90.0 - 21.0);
        tester(180.0, -(90.0 + 21.0));
        tester(45.0, 45.0 - 21.0);
        tester(135.0, -(21.0 + 45.0));
    }

    #[test]
    fn test_traverse_west() {
        let tester = |zero, expected| {
            let g = Gun {
                pos: Pos {
                    x: 10.0,
                    y: 5.0,
                },
                heading_at_zero: Degrees(zero),
            };
            let t = Target {
                pos: Pos {
                    x: 2.0,
                    y: 2.0,
                },
            };
            let (_, trav) = calc(g, t);
            assert_eq!(expected, trav.0.round());
        };
        tester(270.0, 21.0);
        tester(0.0, 21.0 - 90.0);
        tester(180.0, 90.0 + 21.0);
        tester(315.0, 21.0 - 45.0);
        tester(225.0, 45.0 + 21.0);
    }
}

fn main() {
    let g = Gun {
        pos: Pos {
            x: 0.0,
            y: 0.0,
        },
        heading_at_zero: Degrees(180.0),
    };
    let t = Target {
        pos: Pos {
            x: 572.0,
            y: 0.0,
        },
    };
    let (mils, traverse) = calc(g, t);

    println!("Elevation: {:?}", mils);
    println!("Traverse: {:?}", traverse);
}
