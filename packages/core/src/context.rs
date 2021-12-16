use crate::providers::Messagerie;
use crate::providers::Pdfmonkey;
use crate::providers::Pg;
use crate::providers::Sendinblue;
use crate::providers::Stripe;

#[derive(Clone)]
pub struct Context {
    db: Pg,
    pdfmaker: Pdfmonkey,
    mailer: Sendinblue,
    messenger: Messagerie,
    billing_provider: Stripe,
}

impl Context {
    pub fn new(
        pg: Pg,
        pdfmonkey: Pdfmonkey,
        sendinblue: Sendinblue,
        messagerie: Messagerie,
        stripe: Stripe,
    ) -> Self {
        Self {
            db: pg,
            pdfmaker: pdfmonkey,
            mailer: sendinblue,
            messenger: messagerie,
            billing_provider: stripe,
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

    pub fn messenger(&self) -> &Messagerie {
        &self.messenger
    }

    pub fn billing_provider(&self) -> &Stripe {
        &self.billing_provider
    }
}
