use crate::error::no;
use crate::error::Result;
use crate::mailer::Contact;
use crate::mailer::IntoMail;
use serde::Serialize;
use trankeel_data::Amount;
use trankeel_data::DateTime;
use trankeel_data::FileId;
use trankeel_data::FileType;
use trankeel_data::Name;
use trankeel_data::Receipt;
use trankeel_data::Rent;
use trankeel_data::Tenant;
use trankeel_data::Url;
use trankeel_kit::config;
use trankeel_kit::locale;

#[derive(Clone, Default, Debug, Serialize)]
pub struct ReceiptCreatedMail {
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

impl ReceiptCreatedMail {
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
            _recipients: tenants.into_iter().map(Into::into).collect(),
        })
    }
}

impl IntoMail for ReceiptCreatedMail {
    fn template_id(&self) -> u32 {
        config::config()
            .templates("receipt_created_mail")
            .unwrap()
            .id
            .parse::<u32>()
            .unwrap()
    }

    fn subject(&self) -> String {
        if self.is_receipt {
            locale::text("receipt_created_mail.subject")
        } else {
            locale::text("notice_created_mail.subject")
        }
    }

    fn recipients(&self) -> Vec<Contact> {
        self._recipients.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::parse_template;

    #[test]
    fn test_receipt_created_mail() {
        let mail = ReceiptCreatedMail::default();
        let text = config::config()
            .templates("receipt_created_mail")
            .unwrap()
            .as_string()
            .unwrap();

        parse_template(&text)
            .unwrap()
            .render(&liquid::object!({
                "params": {
                    "is_receipt": mail.is_receipt,
                    "name": mail.name,
                    "amount": mail.amount,
                    "charges_amount": mail.charges_amount,
                    "full_amount": mail.full_amount,
                    "period_month": mail.period_month,
                    "period_start": mail.period_start,
                    "period_end": mail.period_end,
                    "download_url": mail.download_url,
                    "file_id": mail.file_id,
                    "date": mail.date,
                },
            }))
            .unwrap();
    }
}
