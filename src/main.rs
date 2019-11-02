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
 * Fix some of the math, depending on vert/hoiz and left/right/up/down and zeroing
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

    pub fn angle(&self, other: &Pos, zero: Degrees) -> Degrees {
        let adjacent = (self.x - other.x).abs();
        let opposite = (self.y - other.y).abs();

        let theta_rad = (zero.0 - 90.0) * std::f64::consts::PI / 180.0;

        // Translate
        let mut x_z = other.x - self.x;
        let mut y_z = other.y - self.y;

        //let t_x = -y_z;
        //let t_y = x_z;

        //x_z = t_x;
        //y_z = t_y;

        // Rotate
        //let x_r = x_z * (cos) + y_z * (sin);
        let x_r = x_z * theta_rad.cos() + y_z * theta_rad.sin();
        //let y_r = -x_z * (sin) + y_z * (cos);
        let y_r = -x_z * theta_rad.sin() + y_z * theta_rad.cos();

        dbg!((x_r, y_r));

        //let val = (((adjacent / opposite).atan2()) * 180.0 / std::f64::consts::PI).abs();
        //let mut val = ((other.y - self.y).atan2(other.x - self.x) * 180.0 / std::f64::consts::PI);
        let mut val = (y_r.atan2(x_r) * 180.0 / std::f64::consts::PI);
        //let slope = (other.y - self.y) / (other.x - self.x);
        //dbg!(&slope);
        //if slope < 0.0 {
        //    val = val;
        //} else {
        //    val = val;
        //}
        //val += 90.0;
        //val *= -1.0;

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

fn calc(gun: Gun, target: Target) -> (Mils, Degrees) {
    let trav;
    //let angle;
    let angle = gun.pos.angle(&target.pos, gun.heading_at_zero);
    /*
    if gun.pos.y > target.pos.y {
        // Shooting north
        angle = gun.pos.angle(&target.pos);
    } else {
        // Shooting south
        angle = Degrees(gun.pos.angle(&target.pos).0 * -1.0);
    }
    */
    dbg!(&angle);
    //if 180.0 < gun.heading_at_zero.0 {
        //trav = 180.0 - gun.heading_at_zero.0 + angle.0;
        //trav = 90.0 + gun.heading_at_zero.0 - angle.0;
        //trav = gun.heading_at_zero.0 + angle.0;
        trav = angle.0;
    //} else {
        //trav = 180.0 + angle.0 - gun.heading_at_zero.0;
        //trav = 420.0;
    //}
    //trav = (180.0 - gun.heading_at_zero.0 + angle.0) % 360.0;

    (dist_to_mils(gun.pos.dist(&target.pos)), Degrees(trav))
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
    fn test_traverse_north_east() {
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
    fn test_traverse_south_east() {
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
        tester(180.0, -21.0);
        tester(270.0, -21.0 - 90.0);
        tester(90.0, -21.0 + 90.0);
        tester(240.0, -21.0 - 60.0);
        tester(135.0, -21.0 + 45.0);
    }
}

fn main() {
    let g = Gun {
        pos: Pos {
            x: 5.0,
            y: 0.0,
        },
        heading_at_zero: Degrees(180.0),
    };
    let t = Target {
        pos: Pos {
            x: 8.0,
            y: 8.0,
        },
    };
    println!("Distance, {:?}", g.pos.dist(&t.pos));
    println!("");
    println!("Mills, {:?}", dist_to_mils(g.pos.dist(&t.pos)));
    //println!("Angle: {:?}", g.pos.angle(&t.pos));
    let angle;
    if g.pos.y > t.pos.y {
        // Shooting north
        angle = g.pos.angle(&t.pos, g.heading_at_zero);
    } else {
        // Shooting south
        angle = Degrees(g.pos.angle(&t.pos, g.heading_at_zero).0 * -1.0);
    }
    println!("Angle: {:?}", angle);
    println!("Traverse, {:?}", g.heading_at_zero.0 + angle.0);
    let trav;
    if 180.0 < g.heading_at_zero.0 {
        trav = 360.0 - g.heading_at_zero.0 + angle.0;
    } else {
        trav = 180.0 + angle.0 - g.heading_at_zero.0;
    }
    println!("Traverse (corrected), {:?}", trav);
}
