/// https://stripe.com/docs/billing/subscriptions/overview#subscription-states
#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[graphql(name = "SubscriptionStatus")]
#[DieselType = "Accountstatus"]
pub enum AccountStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Trialing,
    Unpaid,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[graphql(name = "UserRole")]
#[DieselType = "Personrole"]
pub enum PersonRole {
    Admin,
    Tenant,
    User,
    Viewer,
    Warrant,
}
