use std::time::Duration;

/// Max size in characters for the input. Just one digit below the u64 max
pub const MAX_INPUT_SIZE: usize = 19;

/// Max duration of a message before it's automatically removed from the screen
pub const MAX_MESSAGE_DURATION: Duration = Duration::from_secs(4);
