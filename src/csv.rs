use serde::Serialize;

use crate::ip_info::ValidatorIpData;

pub fn save_to_file(data: Vec<ValidatorIpData>, file_name: &str) -> anyhow::Result<()> {
    let file = std::fs::File::create(file_name)?;
    let mut writer = csv::Writer::from_writer(file);
    for v in data {
        writer.serialize(v)?;
    }
    writer.flush()?;
    Ok(())
}
