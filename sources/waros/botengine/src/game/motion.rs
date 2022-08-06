use super::*;
use nalgebra::{Point2, Rotation2, Vector2};

#[derive(Debug)]
pub struct MotionComponent {
    pub position: Point2<f32>,
    pub origin: Point2<f32>,
    pub distance_along_heading: i32,
    pub speed: i32,
    pub desired_speed: i32,
    pub heading: i32,
    pub collision: Option<CollisionType>,
}

#[derive(Debug)]
pub enum CollisionType {
    Wall(Point2<f32>),
    Player(String),
}

impl MotionComponent {
    pub fn new() -> MotionComponent {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        // starting at 0 sometimes starts at -0.000003059797
        let x: f32 = rng.gen_range(1.0, MAX_X - 1.0);
        let y: f32 = rng.gen_range(1.0, MAX_Y - 1.0);

        MotionComponent {
            position: Point2::new(x, y),
            origin: Point2::new(x, y),
            distance_along_heading: 0,
            speed: 0,
            desired_speed: 0,
            heading: 0,
            collision: None,
        }
    }
}

pub struct MotionSystem {
    logger: Option<Sender<GameEvent>>,
}

impl MotionSystem {
    pub fn new(logger: Option<Sender<GameEvent>>) -> MotionSystem {
        MotionSystem { logger }
    }

    pub fn advance(mc: &mut MotionComponent) {
        // if the previous cycle resulted in a collision, wipe that status
        mc.collision = None;

        Self::update_speed(mc);
        Self::collision_detect_walls(mc);
        Self::update_distance_on_heading(mc);
    }

    pub fn point_along_heading(source: &Point2<f32>, heading: f32, distance: f32) -> Point2<f32> {
        source + (Rotation2::new(heading.to_radians()) * Vector2::x()) * distance
    }

    fn collision_detect_walls(mc: &mut MotionComponent) {
        if mc.position.x < 0.0 {
            mc.collision = Some(CollisionType::Wall(mc.position.clone()));
            mc.position.x = 1.0;
            Self::stop(mc);
        }

        if mc.position.x > MAX_X {
            mc.collision = Some(CollisionType::Wall(mc.position.clone()));
            mc.position.x = MAX_X - 1.0;
            Self::stop(mc);
        }

        if mc.position.y < 0.0 {
            mc.collision = Some(CollisionType::Wall(mc.position.clone()));
            mc.position.y = 1.0;
            Self::stop(mc);
        }
        if mc.position.y > MAX_Y {
            mc.collision = Some(CollisionType::Wall(mc.position.clone()));
            mc.position.y = MAX_Y - 1.0;
            Self::stop(mc);
        }
    }

    fn stop(mc: &mut MotionComponent) {
        mc.speed = 0;
        mc.desired_speed = 0;
    }

    fn update_speed(mc: &mut MotionComponent) {
        if mc.speed != mc.desired_speed {
            if mc.speed > mc.desired_speed {
                // slowing down
                mc.speed -= ACCEL;
                if mc.speed < mc.desired_speed {
                    mc.speed = mc.desired_speed;
                }
            } else {
                // speeding up
                mc.speed += ACCEL;
                if mc.speed > mc.desired_speed {
                    mc.speed = mc.desired_speed;
                }
            }

            if mc.speed < 0 {
                mc.speed = 0;
            }
        }
    }

    fn update_distance_on_heading(mc: &mut MotionComponent) {
        if mc.speed > 0 {
            mc.distance_along_heading += mc.speed * ROBOT_SPEED;
            mc.position = Self::point_along_heading(
                &mc.origin,
                mc.heading as f32,
                mc.distance_along_heading as f32,
            );
        }
    }
}

impl System for MotionSystem {
    fn apply(&self, _cycle: u32, game_state: &Arc<GameState>) {
        game_state.players.read().unwrap().iter().for_each(|p| {
            game_state
                .motion_components
                .write()
                .unwrap()
                .entry(p.to_string())
                .and_modify(|mc| Self::advance(mc));
        });
    }
}

const ROBOT_SPEED: i32 = 1; // multiplicative factor, robot moves at current.speed * ROBOT_SPEED per cycle
const ACCEL: i32 = 5;
pub const MAX_ENGINE: i32 = 100;

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn advance_on_heading() {
        let source = Point2::new(0.0f32, 0.0);
        assert_relative_eq!(
            Point2::new(0.0f32, 10.0),
            MotionSystem::point_along_heading(&source, 90.0, 10.0),
            max_relative = 1.0,
            epsilon = std::f32::EPSILON
        );

        assert_abs_diff_eq!(
            Point2::new(9.192388_f32, 9.192388_f32),
            MotionSystem::point_along_heading(&source, 45.0, 13.0),
        );
    }
}
