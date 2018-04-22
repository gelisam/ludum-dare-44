use checkpoint::*;
use map::Map;
use racer::Racer;


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Action {
    TurnLeft,
    TurnRight,
    GoForward,
}

#[derive(Clone, Copy, Debug)]
pub struct Ai {
    pub racer: Racer,
}

impl Ai {
    pub fn new(racer: Racer) -> Ai {
        Ai {racer}
    }

    pub fn next_action(self) -> Action {
        if self.racer.direction_index == forward(self.racer.position) {
            Action::GoForward
        } else {
            Action::TurnLeft
        }
    }

    pub fn perform_action(self, action: Action, map: &mut Map) -> Ai {
        match action {
            Action::TurnLeft  => Ai {racer: self.racer.turn_left(), ..self},
            Action::TurnRight => Ai {racer: self.racer.turn_right(), ..self},
            Action::GoForward => match self.racer.go_forward(map) {
                Some(racer) => Ai {racer, ..self},
                None        => self,
            },
        }
    }
}
