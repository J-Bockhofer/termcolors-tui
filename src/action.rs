use std::fmt;

use serde::{
  de::{self, Deserializer, Visitor},
  Deserialize, Serialize,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Action {
  Tick,
  Render,
  Resize(u16, u16),
  Suspend,
  Resume,
  Quit,
  Refresh,
  Error(String),
  Help,

  NextColor,
  PreviousColor,

  InputHEX,
  InputRGB,
  InputPrompt,

  SubmitInput,

  ChangeUndo,
  ChangeRedo,
  ShowShades,
  PreviousShade,
  NextShade,

  InvertColor,
  InvertAll,

  SwitchMarker,
  ToggleSpin,

  ToggleHSV,
  HSVPrev,
  HSVNext,
  HSVIncrease,
  HSVDecrease,

  ColorUp,
  ColorDown,

  TogglePalette,
}

impl<'de> Deserialize<'de> for Action {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ActionVisitor;

    impl<'de> Visitor<'de> for ActionVisitor {
      type Value = Action;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid string representation of Action")
      }

      fn visit_str<E>(self, value: &str) -> Result<Action, E>
      where
        E: de::Error,
      {
        match value {
          "Tick" => Ok(Action::Tick),
          "Render" => Ok(Action::Render),
          "Suspend" => Ok(Action::Suspend),
          "Resume" => Ok(Action::Resume),
          "Quit" => Ok(Action::Quit),
          "Refresh" => Ok(Action::Refresh),
          "Help" => Ok(Action::Help),
          "NextColor" => Ok(Action::NextColor),
          "PreviousColor" => Ok(Action::PreviousColor),
          "InputHEX" => Ok(Action::InputHEX),
          "InputRGB" => Ok(Action::InputRGB),
          "Input" => Ok(Action::InputPrompt),
          "SubmitInput" => Ok(Action::SubmitInput),
          "Undo" => Ok(Action::ChangeUndo),
          "Redo" => Ok(Action::ChangeRedo),
          "Shades" => Ok(Action::ShowShades),
          "PreviousShade" => Ok(Action::PreviousShade),
          "NextShade" => Ok(Action::NextShade),
          "InvertColor" => Ok(Action::InvertColor),
          "InvertAll" => Ok(Action::InvertAll),
          "SwitchMarker" => Ok(Action::SwitchMarker),
          "ToggleSpin" => Ok(Action::ToggleSpin),
          "HSV" => Ok(Action::ToggleHSV),
          "HSVPrev" => Ok(Action::HSVPrev),
          "HSVNext" => Ok(Action::HSVNext),
          "HSVDecrease" => Ok(Action::HSVDecrease),
          "HSVIncrease" => Ok(Action::HSVIncrease),
          "ColorUp" => Ok(Action::ColorUp),
          "ColorDown" => Ok(Action::ColorDown),
          "Palette" => Ok(Action::TogglePalette),
          data if data.starts_with("Error(") => {
            let error_msg = data.trim_start_matches("Error(").trim_end_matches(")");
            Ok(Action::Error(error_msg.to_string()))
          },
          data if data.starts_with("Resize(") => {
            let parts: Vec<&str> = data.trim_start_matches("Resize(").trim_end_matches(")").split(',').collect();
            if parts.len() == 2 {
              let width: u16 = parts[0].trim().parse().map_err(E::custom)?;
              let height: u16 = parts[1].trim().parse().map_err(E::custom)?;
              Ok(Action::Resize(width, height))
            } else {
              Err(E::custom(format!("Invalid Resize format: {}", value)))
            }
          },
          _ => Err(E::custom(format!("Unknown Action variant: {}", value))),
        }
      }
    }

    deserializer.deserialize_str(ActionVisitor)
  }
}
