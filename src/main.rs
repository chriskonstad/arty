#[derive(Debug)]
struct Distance(f64);

#[derive(Debug)]
struct Mils(f64);

#[derive(Debug)]
struct Degrees(f64);

/**
 * TODO
 * UX
 * Parsing
 * Something about bounds to 180 degrees for the angle
 */

/*
 * NOTES:
 * Traverse goes: - ==== + (negative to the left, positive to the right)
 * This assumes the standard
 *   A B C
 * 1             N
 * 2           W   E
 * 3             S
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

    pub fn angle(&self, other: &Pos) -> Degrees {
        let adjacent = (self.x - other.x).abs();
        let opposite = (self.y - other.y).abs();

        let mut val = (((adjacent / opposite).atan()) * 180.0 / std::f64::consts::PI).abs();

        // TODO This is wrong but we'll get there :)
        /*
        let multiplier;
        if other.y - self.y < 0.0 {
            if other.x - self.x < 0.0 {
                multiplier = -1.0;
            } else {
                multiplier = 1.0;
            }
        } else {
            if other.x - self.x > 0.0 {
                multiplier = 1.0;
            } else {
                multiplier = -1.0;
            }
        }
        */
        /*
        if 180.0 < val {
            val = -(val - 180.0);
        } else if val < -180.0 {
            val = -(val + 180.0);
        }
        */
        Degrees(val)
        //Degrees(val.abs() * multiplier)
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
}

fn main() {
    let g = Gun {
        pos: Pos {
            x: 5.0,
            y: 10.0,
        },
        heading_at_zero: Degrees(45.0),
    };
    let t = Target {
        pos: Pos {
            x: 8.0,
            y: 2.0,
        },
    };
    println!("Distance, {:?}", g.pos.dist(&t.pos));
    println!("");
    println!("Mills, {:?}", dist_to_mils(g.pos.dist(&t.pos)));
    println!("Angle: {:?}", g.pos.angle(&t.pos));
    println!("Traverse, {:?}", g.heading_at_zero.0 + g.pos.angle(&t.pos).0);
    let trav;
    if 180.0 < g.heading_at_zero.0 {
        trav = 360.0 - g.heading_at_zero.0 + g.pos.angle(&t.pos).0;
    } else {
        trav = g.pos.angle(&t.pos).0 - g.heading_at_zero.0;
    }
    println!("Traverse (corrected), {:?}", trav);
}
