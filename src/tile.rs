#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Value {
    Number(i32),
    Mine,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum State {
    Opened,
    Closed,
    Flagged,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    value: Value,
    state: State,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            value: Value::Number(0),
            state: State::Closed,
        }
    }
}

impl Tile {
    pub fn open(&mut self) -> Result<(), ()> {
        if self.value == Value::Mine {
            return Err(());
        }

        if self.state == State::Closed {
            self.state = State::Opened;
        }

        Ok(())
    }

    pub fn flag(&mut self) {
        if self.state == State::Closed {
            self.state = State::Flagged;
        } else if self.state == State::Flagged {
            self.state = State::Closed;
        }
    }

    pub fn replace_value(&mut self, value: Value) {
        self.value = value;
    }

    pub fn value(&self) -> Value {
        self.value
    }

    pub fn state(&self) -> State {
        self.state
    }
}
