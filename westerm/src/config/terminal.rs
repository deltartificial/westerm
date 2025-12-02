use serde::{Deserialize, Deserializer, Serialize, de};
use toml::Value;

use westerm_config_derive::{ConfigDeserialize, SerdeReplace};
use westerm_terminal::term::Osc52;

use crate::config::ui_config::{Program, StringVisitor};

#[derive(ConfigDeserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Terminal {
    /// OSC52 support mode.
    pub osc52: SerdeOsc52,
    /// Path to a shell program to run on startup.
    pub shell: Option<Program>,
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            osc52: SerdeOsc52::default(),
            // Westerm: Launch tmux by default with new-session -A for attach-or-create
            shell: Some(Program::WithArgs {
                program: "/opt/homebrew/bin/tmux".to_string(),
                args: vec!["new-session".to_string(), "-A".to_string(), "-s".to_string(), "main".to_string()],
            }),
        }
    }
}

#[derive(SerdeReplace, Serialize, Default, Copy, Clone, Debug, PartialEq)]
pub struct SerdeOsc52(pub Osc52);

impl<'de> Deserialize<'de> for SerdeOsc52 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = deserializer.deserialize_str(StringVisitor)?;
        Osc52::deserialize(Value::String(value)).map(SerdeOsc52).map_err(de::Error::custom)
    }
}
