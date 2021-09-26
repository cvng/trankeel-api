use crate::error::Result;
use crate::Amount;
use crate::DateTime;
use crate::FileType;
use crate::LenderWithIdentity;
use crate::Name;
use crate::Property;
use crate::Rent;
use crate::Tenant;
use piteo_core::error::no;
use piteo_core::pdfmaker::IntoDocument;
use piteo_data::Receipt;
use serde::Serialize;

/// Receipt or notice document. https://dashboard.pdfmonkey.io/templates/8269e571-7ece-4f0d-bc37-854d77999e0d
#[derive(Clone, Debug, Serialize)]
pub struct ReceiptDocument {
    is_receipt: bool,

    lender_name: String,
    lender_address_city: String,
    lender_address_line1: String,
    lender_address_line2: Option<String>,
    lender_address_postal_code: String,

    tenant_name: String,

    property_address_city: String,
    property_address_line1: String,
    property_address_line2: Option<String>,
    property_address_postal_code: String,

    rent_amount: Amount,
    rent_charges_amount: Amount,
    rent_full_amount: Amount,

    period_start: DateTime,
    period_end: DateTime,

    date: DateTime,
    _filename: String,
}

// # Impls

impl ReceiptDocument {
    pub fn try_new(
        receipt: Receipt,
        rent: Rent,
        lender: LenderWithIdentity,
        tenants: Vec<Tenant>,
        property: Property,
        date: DateTime,
    ) -> Result<Self> {
        Ok(Self {
            is_receipt: (match receipt.type_ {
                FileType::RentReceipt => Some(true),
                FileType::PaymentNotice => Some(false),
                _ => None,
            })
            .ok_or_else(|| no("receipt.type"))?,

            lender_name: lender.1.display_name(),
            lender_address_city: lender
                .1
                .address()
                .ok_or_else(|| no("lender.address"))?
                .city
                .ok_or_else(|| no("lender.address.city"))?,
            lender_address_line1: lender
                .1
                .address()
                .ok_or_else(|| no("lender.address"))?
                .line1
                .ok_or_else(|| no("lender.address.line1"))?,
            lender_address_line2: lender
                .1
                .address()
                .ok_or_else(|| no("lender.address"))?
                .line2,
            lender_address_postal_code: lender
                .1
                .address()
                .ok_or_else(|| no("lender.address"))?
                .postal_code
                .ok_or_else(|| no("lender.address.postal_code"))?,

            tenant_name: tenants
                .iter()
                .map(|tenant| tenant.display_name())
                .collect::<Vec<_>>()
                .join(", "),

            property_address_city: property
                .address
                .city
                .ok_or_else(|| no("property.address.city"))?,
            property_address_line1: property
                .address
                .line1
                .ok_or_else(|| no("property.address.line1"))?,
            property_address_line2: property.address.line2,
            property_address_postal_code: property
                .address
                .postal_code
                .ok_or_else(|| no("property.address.postal_code"))?,

            rent_amount: rent.amount,
            rent_charges_amount: rent.charges_amount.unwrap_or_default(),
            rent_full_amount: rent.full_amount,

            period_start: rent.period_start,
            period_end: rent.period_end,

            date,
            _filename: receipt.filename.ok_or_else(|| no("receipt.filename"))?,
        })
    }
}

impl IntoDocument for ReceiptDocument {
    fn template_id(&self) -> String {
        "756B77DA-7498-4D5C-ADBE-EF4AC4DBE151".into()
    }

    fn filename(&self) -> String {
        self._filename.clone()
    }
}
