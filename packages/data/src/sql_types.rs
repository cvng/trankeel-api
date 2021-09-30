/// https://stripe.com/docs/billing/subscriptions/overview
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

impl Default for AccountStatus {
    fn default() -> Self {
        Self::Trialing
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[DieselType = "Propertystatus"]
pub enum PropertyStatus {
    ForSale,
    Inactive,
    Rented,
    UnderConstruction,
    Unrented,
}

impl Default for PropertyStatus {
    fn default() -> Self {
        Self::Unrented
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[DieselType = "Rentstatus"]
pub enum RentStatus {
    #[graphql(name = "Pending")]
    Open,
    #[graphql(name = "Settled")]
    Paid,
    #[graphql(name = "Partial")]
    PartiallyPaid,
}

impl Default for RentStatus {
    fn default() -> Self {
        Self::Open
    }
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

impl Default for PersonRole {
    fn default() -> Self {
        Self::Viewer
    }
}
