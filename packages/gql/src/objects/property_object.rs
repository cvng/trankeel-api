use super::Address;
use super::Advertisement;
use super::Lease;
use super::Lender;
use async_graphql::Context;
use async_graphql::Result;
use trankeel::AccountId;
use trankeel::Amount;
use trankeel::Client;
use trankeel::DateTime;
use trankeel::LenderId;
use trankeel::PropertyBuildPeriodType;
use trankeel::PropertyBuildingLegalStatus;
use trankeel::PropertyEnergyClass;
use trankeel::PropertyGasEmission;
use trankeel::PropertyHabitationUsageType;
use trankeel::PropertyId;
use trankeel::PropertyRoomType;
use trankeel::PropertyStatus;
use trankeel::PropertyUsageType;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Property {
    pub id: PropertyId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub address: Address,
    pub build_period: Option<PropertyBuildPeriodType>,
    pub building_legal_status: Option<PropertyBuildingLegalStatus>,
    pub common_spaces: Option<String>,
    pub energy_class: Option<PropertyEnergyClass>,
    pub equipments: Option<String>,
    pub gas_emission: Option<PropertyGasEmission>,
    pub heating_method: Option<PropertyUsageType>,
    pub housing_type: Option<PropertyUsageType>,
    pub name: String,
    pub note: Option<String>,
    pub ntic_equipments: Option<String>,
    pub other_spaces: Option<String>,
    pub tax: Option<Amount>,
    pub room_count: PropertyRoomType,
    pub status: PropertyStatus,
    pub surface: f32,
    pub tenant_private_spaces: Option<String>,
    pub usage_type: Option<PropertyHabitationUsageType>,
    pub water_heating_method: Option<PropertyUsageType>,
    pub lender_id: LenderId,
}

#[async_graphql::ComplexObject]
impl Property {
    async fn expected_rents(&self) -> Option<Amount> {
        None
    }

    async fn collected_rents(&self) -> Option<Amount> {
        None
    }

    async fn lender(&self, ctx: &Context<'_>) -> Result<Lender> {
        Ok(ctx
            .data_unchecked::<Client>()
            .lenders()
            .by_id(&self.lender_id)?
            .into())
    }

    async fn leases(&self, ctx: &Context<'_>) -> Result<Vec<Lease>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .leases()
            .by_property_id(&self.id)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn advertisements(&self, ctx: &Context<'_>) -> Result<Vec<Advertisement>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .advertisements()
            .by_property_id(&self.id)?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

impl From<trankeel::Property> for Property {
    fn from(item: trankeel::Property) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            account_id: item.account_id,
            address: item.address.into(),
            build_period: item.build_period,
            building_legal_status: item.building_legal_status,
            common_spaces: item.common_spaces,
            energy_class: item.energy_class,
            equipments: item.equipments,
            gas_emission: item.gas_emission,
            heating_method: item.heating_method,
            housing_type: item.housing_type,
            name: item.name,
            note: item.note,
            ntic_equipments: item.ntic_equipments,
            other_spaces: item.other_spaces,
            tax: item.tax,
            room_count: item.room_count,
            status: item.status,
            surface: item.surface,
            tenant_private_spaces: item.tenant_private_spaces,
            usage_type: item.usage_type,
            water_heating_method: item.water_heating_method,
            lender_id: item.lender_id,
        }
    }
}
