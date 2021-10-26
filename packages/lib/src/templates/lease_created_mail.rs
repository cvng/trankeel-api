use crate::error::no;
use crate::error::Result;
use trankeel_core::mailer::Contact;
use trankeel_core::mailer::IntoMail;
use trankeel_data::Lease;
use trankeel_data::LeaseFile;
use trankeel_data::Tenant;
use trankeel_data::Url;
use trankeel_kit::config::config;
use trankeel_kit::locale;

#[derive(Clone, Debug, Serialize)]
pub struct LeaseCreatedMail {
    download_url: Url,
    _recipients: Vec<Contact>,
}

impl LeaseCreatedMail {
    pub fn try_new(_lease: &Lease, lease_file: &LeaseFile, tenants: Vec<Tenant>) -> Result<Self> {
        Ok(Self {
            download_url: lease_file
                .download_url
                .clone()
                .ok_or_else(|| no("download_url"))?,
            _recipients: tenants.into_iter().map(Into::into).collect(),
        })
    }
}

impl IntoMail for LeaseCreatedMail {
    fn template_id(&self) -> u32 {
        config()
            .templates("lease_created_mail")
            .unwrap()
            .id
            .parse::<u32>()
            .unwrap()
    }

    fn subject(&self) -> String {
        locale::text("lease_created_mail.subject")
    }

    fn recipients(&self) -> Vec<Contact> {
        self._recipients.clone()
    }
}
