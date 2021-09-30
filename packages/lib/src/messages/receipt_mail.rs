use crate::error::no;
use crate::error::Result;
use piteo_core::mailer::Contact;
use piteo_core::mailer::IntoMail;
use piteo_data::locale;
use piteo_data::Amount;
use piteo_data::DateTime;
use piteo_data::FileId;
use piteo_data::FileType;
use piteo_data::Name;
use piteo_data::Receipt;
use piteo_data::Rent;
use piteo_data::Tenant;
use piteo_data::Url;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ReceiptMail {
    is_receipt: bool,

    name: String,

    amount: Amount,
    charges_amount: Amount,
    full_amount: Amount,

    period_month: String,
    period_start: DateTime,
    period_end: DateTime,

    download_url: Url,
    file_id: FileId,

    date: DateTime,
    _recipients: Vec<Contact>,
}

impl ReceiptMail {
    pub fn try_new(
        receipt: &Receipt,
        rent: &Rent,
        tenants: Vec<Tenant>,
        date: DateTime,
    ) -> Result<Self> {
        Ok(Self {
            is_receipt: (match receipt.type_ {
                FileType::RentReceipt => Some(true),
                FileType::PaymentNotice => Some(false),
                _ => None,
            })
            .ok_or_else(|| no("receipt.type"))?,

            name: tenants
                .iter()
                .map(|tenant| tenant.display_name())
                .collect::<Vec<_>>()
                .join(", "),

            amount: rent.amount,
            charges_amount: rent.charges_amount.unwrap_or_default(),
            full_amount: rent.full_amount,

            period_month: rent.period_start.inner().to_string(),
            period_start: rent.period_start,
            period_end: rent.period_end,

            download_url: receipt
                .clone()
                .download_url
                .ok_or_else(|| no("receipt.download_url"))?,
            file_id: receipt.id,

            date,
            _recipients: tenants
                .iter()
                .map(|tenant| Contact {
                    name: tenant.display_name(),
                    email: tenant.email.clone(),
                })
                .collect(),
        })
    }
}

impl IntoMail for ReceiptMail {
    fn template_id(&self) -> u32 {
        1 // https://my.sendinblue.com/camp/template/1/message-setup
    }

    fn subject(&self) -> String {
        if self.is_receipt {
            // "Votre quittance de loyer est arrivée !"
            locale::text("receipt_mail_subject")
        } else {
            // "Votre avis d'échéance est arrivé !"
            locale::text("notice_mail_subject")
        }
    }

    fn recipients(&self) -> Vec<Contact> {
        self._recipients.clone()
    }
}
