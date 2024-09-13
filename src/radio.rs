use windows::{Devices::Radios::{Radio, RadioKind, RadioState}, Foundation::Collections::IVectorView};

use anyhow::Result;

use colored::*;

pub fn describe_radio(radio: &Radio) -> String {
    let name = radio.Name().unwrap_or("Unknown".into()).to_string().bold();
    
    let kind = match radio.Kind() {
        Ok(RadioKind::Bluetooth) => "Bluetooth".blue(),
        Ok(RadioKind::WiFi) => "WiFi".green(),
        Ok(RadioKind::FM) => "FM".yellow(),
        Ok(RadioKind::MobileBroadband) => "MobileBroadband".magenta(),
        Ok(RadioKind::Other) => "Other".cyan(),
        _ => "Unknown".red(),
    };
    
    let state = match radio.State() {
        Ok(RadioState::On) => "✅ On".green().bold(),
        Ok(RadioState::Off) => "❌ Off".red().bold(),
        Ok(RadioState::Disabled) => "⚠️ Disabled".yellow().bold(),
        _ => "❓ Unknown".red(),
    };

    format!(
        "{}: \"{}\"\n  {}: {}\n  {}: {}",
        "Radio".bold().underline(),
        name,
        "Kind".bold(),
        kind,
        "State".bold(),
        state
    )
}

pub fn describe_radio_plain(radio: &Radio) -> String {
    let name = radio.Name().unwrap_or("Unknown".into()).to_string();
    
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

    format!(
        "{}: \"{}\"\n  {}: {}\n  {}: {}",
        "Radio",
        name,
        "Kind",
        kind,
        "State",
        state
    )
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