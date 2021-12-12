use crate::auth::AddressInput;
use crate::error::Result;
use async_graphql::InputObject;
use trankeel_core::dispatcher::Command;
use trankeel_data::Account;
use trankeel_data::Amount;
use trankeel_data::Lender;
use trankeel_data::LenderId;
use trankeel_data::Property;
use trankeel_data::PropertyBuildPeriodType;
use trankeel_data::PropertyBuildingLegalStatus;
use trankeel_data::PropertyEnergyClass;
use trankeel_data::PropertyGasEmission;
use trankeel_data::PropertyHabitationUsageType;
use trankeel_data::PropertyId;
use trankeel_data::PropertyRoomType;
use trankeel_data::PropertyStatus;
use trankeel_data::PropertyUsageType;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreatePropertyInput {
    pub address: AddressInput,
    pub build_period: Option<PropertyBuildPeriodType>,
    pub building_legal_status: Option<PropertyBuildingLegalStatus>,
    pub common_spaces: Option<String>,
    pub energy_class: Option<PropertyEnergyClass>,
    pub equipments: Option<String>,
    pub gas_emission: Option<PropertyGasEmission>,
    pub heating_method: Option<PropertyUsageType>,
    pub housing_type: Option<PropertyUsageType>,
    pub lender_id: Option<LenderId>,
    pub name: String,
    pub note: Option<String>,
    pub description: Option<String>,
    pub ntic_equipments: Option<String>,
    pub other_spaces: Option<String>,
    pub room_count: Option<PropertyRoomType>,
    pub status: Option<PropertyStatus>,
    pub surface: Option<f32>,
    pub tax: Option<Amount>,
    pub tenant_private_spaces: Option<String>,
    pub usage_type: Option<PropertyHabitationUsageType>,
    pub water_heating_method: Option<PropertyUsageType>,
}

pub struct CreatePropertyPayload {
    pub property: Property,
}

pub(crate) struct CreateProperty {
    account: Account,
    lender: Lender,
}

impl CreateProperty {
    pub fn new(account: &Account, lender: &Lender) -> Self {
        Self {
            account: account.clone(),
            lender: lender.clone(),
        }
    }
}

impl Command for CreateProperty {
    type Input = CreatePropertyInput;
    type Payload = CreatePropertyPayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { account, lender } = self;

        let property = Property {
            id: PropertyId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id: account.id,
            address: input.address.into(),
            build_period: input.build_period,
            building_legal_status: input.building_legal_status,
            common_spaces: input.common_spaces,
            energy_class: input.energy_class,
            equipments: input.equipments,
            gas_emission: input.gas_emission,
            heating_method: input.heating_method,
            housing_type: input.housing_type,
            lender_id: input.lender_id.unwrap_or(lender.id),
            name: input.name,
            note: input.note,
            ntic_equipments: input.ntic_equipments,
            other_spaces: input.other_spaces,
            tax: input.tax,
            room_count: input.room_count,
            status: input.status.unwrap_or_default(),
            surface: input.surface,
            tenant_private_spaces: input.tenant_private_spaces,
            usage_type: input.usage_type,
            water_heating_method: input.water_heating_method,
        };

        Ok(Self::Payload { property })
    }
}
