use crate::config;
use crate::error::no;
use crate::error::Result;
use crate::pdfmaker::IntoDocument;
use trankeel_data::DateTime;
use trankeel_data::Lease;
use trankeel_data::LeaseFile;

#[derive(Clone, Debug, Serialize)]
pub struct LeaseDocument {
    date: DateTime,
    _filename: String,
}

impl LeaseDocument {
    pub fn try_new(_lease: Lease, lease_file: LeaseFile, date: DateTime) -> Result<Self> {
        Ok(LeaseDocument {
            date,
            _filename: lease_file
                .filename
                .ok_or_else(|| no("lease_file.filename"))?,
        })
    }
}

impl IntoDocument for LeaseDocument {
    fn template_id(&self) -> String {
        config::config().templates("lease_document").unwrap().id
    }

    fn filename(&self) -> String {
        self._filename.clone()
    }
}
