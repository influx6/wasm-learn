use super::*;
use nalgebra::{Point2, Rotation2, Vector2};

#[derive(Debug)]
pub struct ScannerComponent {
    pub angle: i32,
}

impl ScannerComponent {
    pub fn new() -> ScannerComponent {
        ScannerComponent { angle: 0 }
    }
}

pub struct ScannerSystem {
    logger: Option<Sender<GameEvent>>,
}

impl ScannerSystem {
    pub fn new(logger: Option<Sender<GameEvent>>) -> ScannerSystem {
        ScannerSystem { logger }
    }

    pub fn scan(game_state: &Arc<GameState>, player: &str, degree: f32, resolution: f32) -> i32 {
        let resolution = resolution.min(RES_LIMIT);

        let mcs = game_state.motion_components.read().unwrap();
        let source = mcs.get(player).unwrap();
        let dcs = game_state.damage_components.read().unwrap();
        let living_players: Vec<_> = game_state
            .players
            .read()
            .unwrap()
            .iter()
            .filter_map(|p| match dcs.get(p) {
                Some(dc) => {
                    if dc.dead() {
                        None
                    } else {
                        Some(p.to_string())
                    }
                }
                None => None,
            })
            .collect();

        let mut targets: Vec<_> = living_players
            .iter()
            .filter(|t| *t != player)
            .filter_map(|t| {
                let target = mcs.get(t).unwrap();
                let heading = Self::heading_to_target(&source.position, &target.position);
                let spread = (heading - degree).abs();

                // is the heading to that target within indicated scan range?
                if spread <= resolution {
                    let r = Self::range_to_target(&source.position, &target.position);
                    if r <= SCAN_MAX_RANGE {
                        Some(r as i32)
                    } else {
                        None // out of range of the scanner
                    }
                } else {
                    None
                }
            })
            .collect();

        targets.sort();
        targets.reverse(); // now have shortest distance to target in scan range
        match targets.first() {
            Some(n) => *n,
            None => 0,
        }
    }

    pub fn heading_to_target(source: &Point2<f32>, target: &Point2<f32>) -> f32 {
        let heading = Rotation2::rotation_between(&Vector2::x(), &(target - source));
        heading.angle().to_degrees()
    }

    pub fn range_to_target(source: &Point2<f32>, target: &Point2<f32>) -> f32 {
        nalgebra::distance(source, target)
    }

    pub fn to_user_heading(real_heading: f32) -> i32 {
        (real_heading + 360.0) as i32 % 360
    }

    pub fn to_real_heading(user_heading: i32) -> i32 {
        if user_heading < 180 {
            user_heading
        } else {
            (user_heading - 360)
        }
    }
}

impl System for ScannerSystem {
    fn apply(&self, _cycle: u32, _game_state: &Arc<GameState>) {}
}

pub const RES_LIMIT: f32 = 10.0;
const SCAN_MAX_RANGE: f32 = 700.0;

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn normalize_headings() {
        assert_eq!(225, ScannerSystem::to_user_heading(-135.0));
        assert_eq!(270, ScannerSystem::to_user_heading(-90.0));
        assert_eq!(135, ScannerSystem::to_user_heading(135.0));

        assert_eq!(-135, ScannerSystem::to_real_heading(225));
        assert_eq!(-90, ScannerSystem::to_real_heading(270));
        assert_eq!(135, ScannerSystem::to_real_heading(135));
        assert_eq!(-45, ScannerSystem::to_real_heading(315));
    }

    #[test]
    fn heading_to_target() {
        let p1 = Point2::new(0.0_f32, 0.0);
        let p2 = Point2::new(100.0_f32, 100.0);

        let heading = ScannerSystem::heading_to_target(&p1, &p2);
        assert_eq!(45.0, heading);

        assert_eq!(
            90.0,
            ScannerSystem::heading_to_target(
                &Point2::new(0.0_f32, 0.0),
                &Point2::new(0.0_f32, 100.0)
            )
        );

        assert_eq!(
            -180.0,
            ScannerSystem::heading_to_target(
                &Point2::new(0.0_f32, 0.0),
                &Point2::new(-100.0_f32, 0.0)
            )
        );

        assert_eq!(
            -135.0,
            ScannerSystem::heading_to_target(
                &Point2::new(0.0_f32, 0.0),
                &Point2::new(-100.0_f32, -100.0)
            )
        );

        assert_eq!(
            -135.0,
            ScannerSystem::heading_to_target(
                &Point2::new(100.0_f32, 100.0),
                &Point2::new(-100.0_f32, -100.0)
            )
        );
    }

    #[test]
    fn distance_to_target() {
        let source = Point2::new(0.0f32, 0.0);
        let target = Point2::new(100.0f32, 100.0);

        let distance = ScannerSystem::range_to_target(&source, &target);
        assert_relative_eq!(141.42136, distance);
    }
}
