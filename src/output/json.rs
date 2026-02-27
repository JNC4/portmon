use crate::data::types::PortEntry;

pub fn print(entries: &[PortEntry]) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(entries)?;
    println!("{json}");
    Ok(())
}
