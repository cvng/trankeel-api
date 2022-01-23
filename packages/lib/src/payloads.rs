use trankeel_data::Account;
use trankeel_data::Lender;
use trankeel_data::Person;

pub struct CreateUserWithAccountPayload {
    pub user: Person,
    pub lender: Lender,
    pub account: Account,
}
