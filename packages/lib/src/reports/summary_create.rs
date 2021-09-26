use crate::error::Result;
use piteo_data::Summary;

pub fn get_summary() -> Result<Summary> {
    Ok(Summary::default())
}
