use regex::Regex;
use std::str::FromStr;

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

const GRID_IN_METERS: i32 = 100;

#[derive(Debug, PartialEq)]
struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn dist(&self, other: &Pos) -> Distance {
        Distance(((self.x - other.x).abs().powi(2) + (self.y - other.y).abs().powi(2)).sqrt())
    }
}

#[derive(Debug)]
struct Gun {
    pub pos: Pos,
    pub heading_at_zero: Degrees,
}

impl Gun {
    pub fn calc(&self, target: &Target) -> (Mils, Degrees) {
        // Align the compass's 0 with the mathematical zero.
        let zero = self.heading_at_zero;
        let theta_rad = (zero.0 - 90.0) * std::f64::consts::PI / 180.0;

        // Translate so the gun is at zero
        let x_z = target.pos.x - self.pos.x;
        let y_z = target.pos.y - self.pos.y;

        // Rotate so we zero on the gun's zero
        let x_r = x_z * theta_rad.cos() + y_z * theta_rad.sin();
        let y_r = -x_z * theta_rad.sin() + y_z * theta_rad.cos();

        (
            dist_to_mils(self.pos.dist(&target.pos)),
            Degrees(y_r.atan2(x_r) * 180.0 / std::f64::consts::PI),
        )
    }
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
        let p0 = Pos { x: 1.0, y: 2.0 };
        let p1 = Pos { x: 4.0, y: 6.0 };
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
                pos: Pos { x: 5.0, y: 10.0 },
                heading_at_zero: Degrees(zero),
            };
            let t = Target {
                pos: Pos { x: 8.0, y: 2.0 },
            };
            let (_, trav) = g.calc(&t);
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
                pos: Pos { x: 5.0, y: 0.0 },
                heading_at_zero: Degrees(zero),
            };
            let t = Target {
                pos: Pos { x: 8.0, y: 8.0 },
            };
            let (_, trav) = g.calc(&t);
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
                pos: Pos { x: 0.0, y: 5.0 },
                heading_at_zero: Degrees(zero),
            };
            let t = Target {
                pos: Pos { x: 8.0, y: 2.0 },
            };
            let (_, trav) = g.calc(&t);
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
                pos: Pos { x: 10.0, y: 5.0 },
                heading_at_zero: Degrees(zero),
            };
            let t = Target {
                pos: Pos { x: 2.0, y: 2.0 },
            };
            let (_, trav) = g.calc(&t);
            assert_eq!(expected, trav.0.round());
        };
        tester(270.0, 21.0);
        tester(0.0, 21.0 - 90.0);
        tester(180.0, 90.0 + 21.0);
        tester(315.0, 21.0 - 45.0);
        tester(225.0, 45.0 + 21.0);
    }

    #[test]
    fn test_keypad_to_coord() {
        let scale = GRID_IN_METERS;
        let mini_unit = scale / 6;
        let coord = KeypadCoord {
            grid_right: b'c',
            grid_down: 2,
            keypad: Keypad::ONE,
        };

        assert_eq!(
            Pos {
                x: (scale * 2 + mini_unit) as f64,
                y: (scale * 1 + mini_unit) as f64,
            },
            coord.into()
        );

        let coord2 = KeypadCoord {
            grid_right: b'c',
            grid_down: 2,
            keypad: Keypad::SIX,
        };
        assert_eq!(
            Pos {
                x: (scale * 2 + 5 * mini_unit) as f64,
                y: (scale * 1 + 3 * mini_unit) as f64,
            },
            coord2.into()
        );
    }

    #[test]
    fn test_parse_keypad() {
        assert_eq!(
            KeypadCoord {
                grid_right: b'c',
                grid_down: 2,
                keypad: Keypad::ONE,
            },
            KeypadCoord::from_str("c2k1").unwrap()
        );

        assert_eq!(
            KeypadCoord {
                grid_right: b'd',
                grid_down: 7,
                keypad: Keypad::FIVE,
            },
            KeypadCoord::from_str("D7K5").unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_keypad_keypad_10() {
        KeypadCoord::from_str("c2k10").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_keypad_keypad_0() {
        KeypadCoord::from_str("c2k0").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_keypad_bad_keypad_grid_letter() {
        KeypadCoord::from_str("$2k1").unwrap();
    }

    #[test]
    fn test_parse_mgrs() {
        assert_eq!(
            MgrsCoord {
                grid_right: b'e',
                grid_down: 4,
                easting: 51,
                northing: 78,
            },
            MgrsCoord::from_str("E45178").unwrap()
        );

        assert_eq!(
            MgrsCoord {
                grid_right: b'f',
                grid_down: 8,
                easting: 102,
                northing: 071,
            },
            MgrsCoord::from_str("f8102071").unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_mgrs_unequal_coords() {
        MgrsCoord::from_str("f810207").unwrap();
    }

    #[test]
    fn test_mgrs_to_coord() {
        let scale = GRID_IN_METERS;
        let coord = MgrsCoord {
            grid_right: b'c',
            grid_down: 2,
            easting: 51,
            northing: 78,
        };

        assert_eq!(
            Pos {
                x: (scale * 2 + coord.easting) as f64,
                y: (scale * 2 - coord.northing) as f64,
            },
            coord.into()
        );
    }
}

#[derive(Debug, PartialEq)]
enum Keypad {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
}

impl std::str::FromStr for Keypad {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "1" => Ok(Keypad::ONE),
            "2" => Ok(Keypad::TWO),
            "3" => Ok(Keypad::THREE),
            "4" => Ok(Keypad::FOUR),
            "5" => Ok(Keypad::FIVE),
            "6" => Ok(Keypad::SIX),
            "7" => Ok(Keypad::SEVEN),
            "8" => Ok(Keypad::EIGHT),
            "9" => Ok(Keypad::NINE),
            _ => Err(anyhow::anyhow!("Keypad not in range 1-9!")),
        }
    }
}

#[derive(Debug, PartialEq)]
struct KeypadCoord {
    pub grid_right: u8,
    pub grid_down: i32,
    pub keypad: Keypad,
}

impl From<KeypadCoord> for Pos {
    fn from(coord: KeypadCoord) -> Self {
        // Convert it to the middle of the keypad
        let chunk = GRID_IN_METERS / 6;
        let x_mult = match coord.keypad {
            Keypad::ONE | Keypad::FOUR | Keypad::SEVEN => 0,
            Keypad::TWO | Keypad::FIVE | Keypad::EIGHT => 2,
            Keypad::THREE | Keypad::SIX | Keypad::NINE => 4,
        };
        let x = ((coord.grid_right - b'a') as i32) * GRID_IN_METERS + (x_mult * chunk) + chunk;
        let y_mult = match coord.keypad {
            Keypad::ONE | Keypad::TWO | Keypad::THREE => 0,
            Keypad::FOUR | Keypad::FIVE | Keypad::SIX => 2,
            Keypad::SEVEN | Keypad::EIGHT | Keypad::NINE => 4,
        };
        let y = (coord.grid_down - 1) * GRID_IN_METERS + (y_mult * chunk) + chunk;

        Pos {
            x: x as f64,
            y: y as f64,
        }
    }
}

impl FromStr for KeypadCoord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cleaned = s.trim().to_string();
        cleaned.make_ascii_lowercase();
        let re = Regex::new(r"^([a-z])(\d)k([1-9])$").unwrap();
        let cap = re
            .captures(&cleaned)
            .ok_or(anyhow::anyhow!("Unable to parse as KeypadCoord"))?;

        let right = cap
            .get(1)
            .ok_or(anyhow::anyhow!("Missing grid letter"))?
            .as_str()
            .as_bytes()
            .first()
            .unwrap();
        let down = cap
            .get(2)
            .ok_or(anyhow::anyhow!("Missing grid number"))?
            .as_str()
            .parse::<i32>()?;
        let keypad = cap
            .get(3)
            .ok_or(anyhow::anyhow!("Missing keypad"))?
            .as_str()
            .parse::<Keypad>()?;

        Ok(KeypadCoord {
            grid_right: *right,
            grid_down: down,
            keypad: keypad,
        })
    }
}

#[derive(Debug, PartialEq)]
struct MgrsCoord {
    pub grid_right: u8,
    pub grid_down: i32,
    pub easting: i32,
    pub northing: i32,
}

impl From<MgrsCoord> for Pos {
    fn from(coord: MgrsCoord) -> Self {
        let x = ((coord.grid_right - b'a') as i32) * GRID_IN_METERS + coord.easting;
        // North is in the -y direction, so add one GRID length and subtract northing
        let y = (coord.grid_down) * GRID_IN_METERS - coord.northing;
        Pos {
            x: x as f64,
            y: y as f64,
        }
    }
}

impl FromStr for MgrsCoord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cleaned = s.trim().to_string();
        cleaned.make_ascii_lowercase();
        let re = Regex::new(r"^(([a-z])(\d)(\d{2})(\d{2}))$|^(([a-z])(\d)(\d{3})(\d{3}))$").unwrap();
        let cap = re
            .captures(&cleaned)
            .ok_or(anyhow::anyhow!("Unable to parse as MgrsCoord"))?;

        dbg!(&cap);

        let offset = if cap.get(1).is_some() {
            1
        } else {
            6
        };

        let right = cap
            .get(1 + offset)
            .ok_or(anyhow::anyhow!("Missing grid letter"))?
            .as_str()
            .as_bytes()
            .first()
            .unwrap();
        let down = cap
            .get(2 + offset)
            .ok_or(anyhow::anyhow!("Missing grid number"))?
            .as_str()
            .parse::<i32>()?;
        let easting = cap
            .get(3 + offset)
            .ok_or(anyhow::anyhow!("Missing easting"))?
            .as_str()
            .parse::<i32>()?;
        let northing = cap
            .get(4 + offset)
            .ok_or(anyhow::anyhow!("Missing northing"))?
            .as_str()
            .parse::<i32>()?;

        Ok(MgrsCoord {
            grid_right: *right,
            grid_down: down,
            easting: easting,
            northing: northing,
        })
    }
}

fn main() {
    let g = Gun {
        pos: Pos { x: 0.0, y: 0.0 },
        heading_at_zero: Degrees(180.0),
    };
    let t = Target {
        pos: Pos { x: 572.0, y: 0.0 },
    };
    let (mils, traverse) = g.calc(&t);

    println!("Elevation: {:?}", mils);
    println!("Traverse: {:?}", traverse);
}
