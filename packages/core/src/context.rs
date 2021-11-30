use crate::providers::Pdfmonkey;
use crate::providers::Pg;
use crate::providers::Sendinblue;
use crate::providers::Stripe;

pub struct Context {
    db: Pg,
    pdfmaker: Pdfmonkey,
    mailer: Sendinblue,
    billing_provider: Stripe,
}

impl Context {
    pub fn new(pg: Pg, pdfmonkey: Pdfmonkey, sendinblue: Sendinblue, stripe: Stripe) -> Self {
        Self {
            db: pg,
            pdfmaker: pdfmonkey,
            mailer: sendinblue,
            billing_provider: stripe,
        }
    }

    pub fn env() -> Self {
        Self {
            db: Pg::init(),
            pdfmaker: Pdfmonkey::init(),
            mailer: Sendinblue::init(),
            billing_provider: Stripe::init(),
        }
    }

    pub fn db(&self) -> &Pg {
        &self.db
    }

    pub fn pdfmaker(&self) -> &Pdfmonkey {
        &self.pdfmaker
    }

    pub fn mailer(&self) -> &Sendinblue {
        &self.mailer
    }

    pub fn billing_provider(&self) -> &Stripe {
        &self.billing_provider
    }
}
