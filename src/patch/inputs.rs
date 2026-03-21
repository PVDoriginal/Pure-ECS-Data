use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Input {
    pub keys: Vec<KeyCode>,
    pub once: bool,
    pub input: fn(ButtonInput<KeyCode>) -> bool,
}

impl Default for Input {
    fn default() -> Input {
        Input {
            keys: vec![],
            once: false,
            input: |_| false,
        }
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.keys == other.keys && self.once == other.once
    }
}

impl Eq for Input {}

#[macro_export]
macro_rules! keys_internal {
    ($keys1:ident, $last:ident) => {
        $keys1.pressed(KeyCode::$last)
    };
    ($keys1:ident, $head:ident $($rest:ident)*) => {
        $keys1.pressed(KeyCode::$head) && keys_internal!($keys1, $($rest)*)
    };
}

#[macro_export]
macro_rules! keys {
    ($first:ident) => {
        |keys1| keys1.pressed(KeyCode::$first)
    };
    ($first:ident, $($rest:ident),*) => {
        |keys1| keys1.pressed(KeyCode::$first) && keys_internal!(keys1, $($rest)*)
    };
}

#[macro_export]
macro_rules! keys_once_internal {
    ($keys1:ident, $last:ident) => {
        $keys1.just_pressed(KeyCode::$last)
    };
    ($keys1:ident, $head:ident $($rest:ident)*) => {
        $keys1.pressed(KeyCode::$head) && keys_once_internal!($keys1, $($rest)*)
    };
}

#[macro_export]
macro_rules! keys_once {
    ($first:ident) => {
        |keys1| keys1.just_pressed(KeyCode::$first)
    };
    ($first:ident, $($rest:ident),*) => {
        |keys1| keys1.pressed(KeyCode::$first) && keys_once_internal!(keys1, $($rest)*)
    };
}
