use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching(None)?;
    manager.set_input_value_matching(None)?;
    manager.set_input_value_matching_multiple(&[])?;

    println!("manager currently sees {} devices", manager.devices().len());
    Ok(())
}
