use crate::providers::Messagerie;
use crate::providers::Pdfmonkey;
use crate::providers::Pg;
use crate::providers::Sendinblue;
use crate::providers::Stripe;
use trankeel_kit::config::Config;

#[derive(Clone)]
pub struct Context {
    config: Config,
    db: Pg,
    pdfmaker: Pdfmonkey,
    mailer: Sendinblue,
    messenger: Messagerie,
    billing_provider: Stripe,
}

impl Context {
    pub fn new(
        config: Config,
        pg: Pg,
        pdfmonkey: Pdfmonkey,
        sendinblue: Sendinblue,
        messagerie: Messagerie,
        stripe: Stripe,
    ) -> Self {
        Self {
            config,
            db: pg,
            pdfmaker: pdfmonkey,
            mailer: sendinblue,
            messenger: messagerie,
            billing_provider: stripe,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
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
