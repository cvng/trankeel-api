use crate::schema::tenants;
use crate::Amount;
use crate::Id;
use crate::TenantId;

pub type BalanceId = Id;

#[derive(Clone, Debug, Default, Identifiable, Queryable, SimpleObject)]
pub struct Balance {
    pub id: BalanceId,
    pub tenant_id: TenantId,
    pub balance: Amount,
}

table! {
    balances (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        balance -> Numeric,
    }
}

joinable!(balances -> tenants (tenant_id));

allow_tables_to_appear_in_same_query!(balances, tenants,);
