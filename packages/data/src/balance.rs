use crate::id;
use crate::sql_schema::persons;
use crate::sql_schema::tenants;
use crate::Amount;
use crate::TenantId;
use async_graphql::SimpleObject;
use fake::Fake;

id!(BalanceId);

#[derive(Clone, Default, Debug, Identifiable, Queryable, SimpleObject)]
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

allow_tables_to_appear_in_same_query!(balances, persons,);
