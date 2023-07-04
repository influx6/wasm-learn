use std::collections::HashMap;
use web_sys::HtmlImageElement;

use self::red_hat_boy_states::*;
use crate::engine;
use crate::models;

mod red_hat_boy_states {
    use crate::models::Point;

    pub const FLOOR: i16 = 479;
    pub const STARTING_POINT: i16 = -20;

    const GRAVITY: i16 = 1;
    const JUMP_SPEED: i16 = -25;

    const IDLE_FRAMES: u8 = 29;
    const RUNNING_FRAMES: u8 = 23;
    const SLIDING_FRAMES: u8 = 14;
    const STANDING_FRAMES: u8 = 14;
    const JUMPING_FRAMES: u8 = 25;
    const RUNNING_SPEED: i16 = 3;

    const IDLE_FRAME_NAME: &str = "Idle";
    const JUMP_FRAME_NAME: &str = "Jump";
    const SLIDING_FRAME_NAME: &str = "Slide";
    const RUN_FRAME_NAME: &str = "Run";

    #[derive(Clone, Copy, Debug)]
    pub struct RedHatBoyContext {
        pub frame: u8,
        pub position: Point,
        pub velocity: Point,
    }

    impl RedHatBoyContext {
        fn update(mut self, frame_count: u8) -> Self {
            self.velocity.y += GRAVITY;
            if self.frame < frame_count {
                self.frame += 1;
            } else {
                self.frame = 0;
            }
            self.position.x += self.velocity.x;
            self.position.y += self.velocity.y;

            if self.position.y > FLOOR {
                self.position.y = FLOOR;
            }
            self
        }

        fn set_vertical_velocity(mut self, y: i16) -> Self {
            self.velocity.y = y;
            self
        }

        fn reset_frame(mut self) -> Self {
            self.frame = 0;
            self
        }

        fn run_up(mut self) -> Self {
            self.velocity.y -= RUNNING_SPEED;
            self
        }

        fn run_down(mut self) -> Self {
            self.velocity.y += RUNNING_SPEED;
            self
        }

        fn run_right(mut self) -> Self {
            self.velocity.x += RUNNING_SPEED;
            self
        }

        fn run_left(mut self) -> Self {
            self.velocity.x -= RUNNING_SPEED;
            self
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct RedHatBoyState<S> {
        context: RedHatBoyContext,
        _state: S,
    }

    impl<S> RedHatBoyState<S> {
        pub fn context(&self) -> &RedHatBoyContext {
            &self.context
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Running;

    impl RedHatBoyState<Running> {
        pub fn frame_name(&self) -> &str {
            RUN_FRAME_NAME
        }

        pub fn update(mut self) -> Self {
            self.context = self.context.update(RUNNING_FRAMES);
            self
        }

        pub fn slide(self) -> RedHatBoyState<Sliding> {
            RedHatBoyState {
                context: self.context.reset_frame(),
                _state: Sliding {},
            }
        }

        pub fn jump(self) -> RedHatBoyState<Jumping> {
            RedHatBoyState {
                context: self.context.set_vertical_velocity(JUMP_SPEED).reset_frame(),
                _state: Jumping {},
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Jumping;

    #[derive(Clone, Copy, Debug)]
    pub enum JumpingEndState {
        Complete(RedHatBoyState<Running>),
        Jumping(RedHatBoyState<Jumping>),
    }

    impl RedHatBoyState<Jumping> {
        pub fn frame_name(&self) -> &str {
            JUMP_FRAME_NAME
        }

        pub fn update(mut self) -> JumpingEndState {
            self.context = self.context.update(JUMPING_FRAMES);
            if self.context.position.y >= FLOOR {
                JumpingEndState::Complete(self.land())
            } else {
                JumpingEndState::Jumping(self)
            }
        }

        pub fn land(self) -> RedHatBoyState<Running> {
            RedHatBoyState {
                context: self.context.reset_frame(),
                _state: Running {},
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub enum SlidingEndState {
        Complete(RedHatBoyState<Running>),
        Sliding(RedHatBoyState<Sliding>),
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Sliding;

    impl RedHatBoyState<Sliding> {
        pub fn frame_name(&self) -> &str {
            SLIDING_FRAME_NAME
        }

        pub fn update(mut self) -> SlidingEndState {
            self.context = self.context.update(SLIDING_FRAMES);
            if self.context.frame >= SLIDING_FRAMES {
                SlidingEndState::Complete(self.stand())
            } else {
                SlidingEndState::Sliding(self)
            }
        }

        pub fn stand(self) -> RedHatBoyState<Running> {
            RedHatBoyState {
                context: self.context.reset_frame(),
                _state: Running {},
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Idle;

    impl RedHatBoyState<Idle> {
        pub fn frame_name(&self) -> &str {
            IDLE_FRAME_NAME
        }

        pub fn update(mut self) -> Self {
            self.context = self.context.update(IDLE_FRAMES);
            self
        }

        pub fn new() -> Self {
            RedHatBoyState {
                _state: Idle {},
                context: RedHatBoyContext {
                    frame: 0,
                    position: Point {
                        x: STARTING_POINT,
                        y: FLOOR,
                    },
                    velocity: Point { x: 0, y: 0 },
                },
            }
        }

        pub fn run_right(self) -> RedHatBoyState<Running> {
            RedHatBoyState {
                context: self.context.reset_frame().run_right(),
                _state: Running {},
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Sliding(RedHatBoyState<Sliding>),
    Jumping(RedHatBoyState<Jumping>),
    Running(RedHatBoyState<Running>),
}

impl From<JumpingEndState> for RedHatBoyStateMachine {
    fn from(state: JumpingEndState) -> Self {
        match state {
            JumpingEndState::Complete(state) => state.into(),
            JumpingEndState::Jumping(state) => state.into(),
        }
    }
}

impl From<SlidingEndState> for RedHatBoyStateMachine {
    fn from(state: SlidingEndState) -> Self {
        match state {
            SlidingEndState::Complete(running_state) => running_state.into(),
            SlidingEndState::Sliding(sliding_state) => sliding_state.into(),
        }
    }
}

impl From<RedHatBoyState<Idle>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Idle>) -> Self {
        RedHatBoyStateMachine::Idle(state)
    }
}

impl From<RedHatBoyState<Jumping>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Jumping>) -> Self {
        RedHatBoyStateMachine::Jumping(state)
    }
}

impl From<RedHatBoyState<Sliding>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Sliding>) -> Self {
        RedHatBoyStateMachine::Sliding(state)
    }
}

impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Running>) -> Self {
        RedHatBoyStateMachine::Running(state)
    }
}

pub enum Event {
    Slide,
    Update,
    Jump,
    RunRight,
    RunLeft,
    RunUp,
    RunDown,
}

impl RedHatBoyStateMachine {
    fn context(&self) -> &RedHatBoyContext {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.context(),
            RedHatBoyStateMachine::Running(state) => &state.context(),
            RedHatBoyStateMachine::Sliding(state) => &state.context(),
            RedHatBoyStateMachine::Jumping(state) => &state.context(),
        }
    }

    fn update(self) -> Self {
        self.transition(Event::Update)
    }

    fn frame_name(&self) -> &str {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.frame_name(),
            RedHatBoyStateMachine::Running(state) => &state.frame_name(),
            RedHatBoyStateMachine::Sliding(state) => &state.frame_name(),
            RedHatBoyStateMachine::Jumping(state) => &state.frame_name(),
        }
    }

    fn transition(self, event: Event) -> Self {
        match (self, event) {
            (RedHatBoyStateMachine::Idle(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Running(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Sliding(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Jumping(state), Event::Update) => state.update().into(),

            (RedHatBoyStateMachine::Running(state), Event::Slide) => state.slide().into(),
            (RedHatBoyStateMachine::Running(state), Event::Jump) => state.jump().into(),
            (RedHatBoyStateMachine::Idle(state), Event::RunRight) => state.run_right().into(),

            // (RedHatBoyStateMachine::Idle(state), Event::RunUp) => state.run_up().into(),
            // (RedHatBoyStateMachine::Idle(state), Event::RunDown) => state.run_down().into(),
            // (RedHatBoyStateMachine::Idle(state), Event::RunLeft) => state.run_left().into(),
            _ => self,
        }
    }
}

pub struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite_sheet: models::Sheet,
    image: HtmlImageElement,
}

impl RedHatBoy {
    pub fn new(sheet: models::Sheet, image: HtmlImageElement) -> Self {
        RedHatBoy {
            state_machine: RedHatBoyState::new().into(),
            sprite_sheet: sheet,
            image,
        }
    }

    pub fn update(&mut self) {
        self.state_machine = self.state_machine.update();
    }

    pub fn run_up(&mut self) {
        self.state_machine = self.state_machine.transition(Event::RunUp);
    }

    pub fn jump(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Jump);
    }

    pub fn slide(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Slide);
    }

    pub fn run_down(&mut self) {
        self.state_machine = self.state_machine.transition(Event::RunDown);
    }

    pub fn run_right(&mut self) {
        self.state_machine = self.state_machine.transition(Event::RunRight);
    }

    pub fn run_left(&mut self) {
        self.state_machine = self.state_machine.transition(Event::RunLeft);
    }

    fn frame_name(&self) -> String {
        format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            (self.state_machine.context().frame / 3) + 1,
        )
    }

    fn current_sprite(&self) -> Option<&models::Cell> {
        self.sprite_sheet.frames.get(&self.frame_name())
    }

    fn bounding_box(&self) -> models::Rect {
        let sprite = self.current_sprite().expect("Cell not found");

        models::Rect {
            x: (self.state_machine.context().position.x + sprite.sprite_source_size.x as i16)
                .into(),
            y: (self.state_machine.context().position.y + sprite.sprite_source_size.y as i16)
                .into(),
            width: sprite.frame.w.into(),
            height: sprite.frame.h.into(),
        }
    }

    pub fn draw(&self, renderer: &engine::Renderer) {
        let sprite = self.current_sprite().expect("Cell not found");

        renderer.draw_image(
            &self.image,
            &models::Rect {
                x: sprite.frame.x.into(),
                y: sprite.frame.y.into(),
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            },
            &models::Rect {
                x: (self.state_machine.context().position.x + sprite.sprite_source_size.x as i16)
                    .into(),
                y: (self.state_machine.context().position.y + sprite.sprite_source_size.y as i16)
                    .into(),
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            },
        );
    }
}
