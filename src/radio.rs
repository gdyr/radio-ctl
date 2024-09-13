use windows::{Devices::Radios::{Radio, RadioKind, RadioState}, Foundation::Collections::IVectorView};

use anyhow::Result;

pub fn describe_radio(radio: &Radio) -> String {
  let name = radio.Name().unwrap_or("Unknown".into());
  let kind = match radio.Kind() {
      Ok(RadioKind::Bluetooth) => "Bluetooth",
      Ok(RadioKind::WiFi) => "WiFi",
      Ok(RadioKind::FM) => "FM",
      Ok(RadioKind::MobileBroadband) => "MobileBroadband",
      Ok(RadioKind::Other) => "Other",
      _ => "Unknown",
  };
  let state = match radio.State() {
      Ok(RadioState::On) => "On",
      Ok(RadioState::Off) => "Off",
      Ok(RadioState::Disabled) => "Disabled",
      _ => "Unknown",
  };
  format!("Radio: {:?}\n  kind: {:?}\n  state: {:?}", name, kind, state)
}

pub fn get_radios() -> Result<IVectorView<Radio>> {
  Ok(Radio::GetRadiosAsync()?.get()?)
}

pub fn turn_on_radio(radio: &Radio) -> Result<()> {
  radio.SetStateAsync(RadioState::On)?.get()?;
  Ok(())
}

pub fn turn_off_radio(radio: &Radio) -> Result<()> {
  radio.SetStateAsync(RadioState::Off)?.get()?;
  Ok(())
}