use crate::auth::ops::AddressInput;
use crate::database::Db;
use crate::AuthId;
use eyre::Error;
use piteo_data::Amount;
use piteo_data::LenderId;
use piteo_data::Property;
use piteo_data::PropertyBuildPeriodType;
use piteo_data::PropertyBuildingLegalStatus;
use piteo_data::PropertyEnergyClass;
use piteo_data::PropertyGasEmission;
use piteo_data::PropertyHabitationUsageType;
use piteo_data::PropertyRoomType;
use piteo_data::PropertyStatus;
use piteo_data::PropertyUsageType;
use validator::Validate;

// # Input

#[derive(Validate)]
pub struct CreatePropertyInput {
    pub address: AddressInput,
    pub build_period: PropertyBuildPeriodType,
    pub building_legal_status: PropertyBuildingLegalStatus,
    pub common_spaces: Option<String>,
    pub energy_class: Option<PropertyEnergyClass>,
    pub equipments: Option<String>,
    pub gas_emission: Option<PropertyGasEmission>,
    pub heating_method: PropertyUsageType,
    pub housing_type: PropertyUsageType,
    pub lender_id: LenderId,
    pub name: String,
    pub note: Option<String>,
    pub ntic_equipments: Option<String>,
    pub other_spaces: Option<String>,
    pub room_count: PropertyRoomType,
    pub status: Option<PropertyStatus>,
    pub surface: f64,
    pub tax: Option<Amount>,
    pub tenant_private_spaces: Option<String>,
    pub usage_type: PropertyHabitationUsageType,
    pub water_heating_method: PropertyUsageType,
}

// # Operation

pub fn create_property(
    db: impl Db,
    auth_id: AuthId,
    input: CreatePropertyInput,
) -> Result<Property, Error> {
    input.validate()?;

    let account = db.accounts().by_auth_id(auth_id)?;

    db.properties().create(Property {
        account_id: Some(account.id),
        address: input.address.into(),
        build_period: Some(input.build_period),
        building_legal_status: Some(input.building_legal_status),
        common_spaces: input.common_spaces,
        energy_class: input.energy_class,
        equipments: input.equipments,
        gas_emission: input.gas_emission,
        heating_method: Some(input.heating_method),
        housing_type: Some(input.housing_type),
        lender_id: input.lender_id,
        name: input.name,
        note: input.note,
        ntic_equipments: input.ntic_equipments,
        other_spaces: input.other_spaces,
        tax: input.tax,
        room_count: input.room_count,
        status: input.status,
        surface: input.surface,
        tenant_private_spaces: input.tenant_private_spaces,
        usage_type: Some(input.usage_type),
        water_heating_method: Some(input.water_heating_method),
        id: Default::default(),
    })
}
