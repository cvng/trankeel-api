use eyre::Error;
use piteo_data::Summary;

pub fn get_summary() -> Result<Summary, Error> {
    Ok(Summary::default())
}
