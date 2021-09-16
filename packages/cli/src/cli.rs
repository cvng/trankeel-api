use chrono::Utc;
use piteo::auth::AddressInput;
use piteo::auth::CreateUserWithAccountInput;
use piteo::database::Db;
use piteo::leases::CreateFurnishedLeaseInput;
use piteo::properties::CreatePropertyInput;
use piteo::tenants::CreateTenantInput;
use piteo::Amount;
use piteo::AuthId;
use piteo::PropertyBuildPeriodType;
use piteo::PropertyBuildingLegalStatus;
use piteo::PropertyHabitationUsageType;
use piteo::PropertyRoomType;
use piteo::PropertyUsageType;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let cmd = args[1].as_str();

    match cmd {
        "seed" => seed().await,
        "generate" => write_schema().await,
        _ => eprintln!("error: invalid command"),
    }
}

async fn write_schema() {
    piteo_graphql::write_schema("schema.graphql").ok();
    println!("💫 GraphQL schema printed.");
}

async fn seed() {
    let url = env::var("DATABASE_URL").unwrap();

    let db_pool = piteo::build_connection_pool(&url).unwrap();
    let auth_id = AuthId::new(env::var("DEBUG_AUTH_ID").unwrap());

    let user = piteo::create_user_with_account(
        db_pool.clone(),
        CreateUserWithAccountInput {
            auth_id: auth_id.clone(),
            email: "dev@piteo.fr".into(),
            first_name: "Dev".into(),
            last_name: "PITEO".into(),
            address: None,
            skip_create_customer: Some(true),
        },
    )
    .await
    .unwrap();

    let lender = piteo::db(db_pool.clone())
        .lenders()
        .by_individual_id(user.id)
        .unwrap();

    let property = piteo::create_property(
        db_pool.clone(),
        auth_id.clone(),
        CreatePropertyInput {
            address: AddressInput {
                city: "Talence".into(),
                line1: "27 Rue de la petite mission".into(),
                line2: Some("Etg 1 apt 12".into()),
                postal_code: "16000".into(),
                country: Default::default(),
            },
            build_period: PropertyBuildPeriodType::BeforeY1949,
            building_legal_status: PropertyBuildingLegalStatus::Copro,
            common_spaces: None,
            energy_class: None,
            equipments: None,
            gas_emission: None,
            heating_method: PropertyUsageType::Collective,
            housing_type: PropertyUsageType::Individual,
            lender_id: lender.id(),
            name: "Petite mission".into(),
            note: Some("RAS".into()),
            ntic_equipments: None,
            other_spaces: None,
            room_count: PropertyRoomType::T1,
            status: None,
            surface: 20.0,
            tax: None,
            tenant_private_spaces: None,
            usage_type: PropertyHabitationUsageType::Habitation,
            water_heating_method: PropertyUsageType::Individual,
        },
    )
    .unwrap();

    let tenant = piteo::create_tenant(
        db_pool.clone(),
        auth_id.clone(),
        CreateTenantInput {
            apl: None,
            birthdate: Utc::now().into(),
            birthplace: None,
            email: "tenant@piteo.dev".into(),
            first_name: "Tenant".into(),
            last_name: "PITEO".into(),
            note: None,
            phone_number: None,
            visale_id: None,
        },
    )
    .unwrap();

    let lease = piteo::create_furnished_lease(
        db_pool,
        auth_id,
        CreateFurnishedLeaseInput {
            details: None,
            deposit_amount: None,
            effect_date: Utc::now().into(),
            renew_date: None,
            file: None,
            property_id: property.id,
            rent_amount: Amount::new(360, 0),
            rent_charges_amount: Some(Amount::new(90, 0)),
            signature_date: None,
            tenant_ids: vec![tenant.id],
        },
    )
    .unwrap();

    println!(
        "{:#?}\n{:#?}\n{:#?}\n{:#?}\n{:#?}",
        user, lender, property, tenant, lease
    );
    println!("🌱 Database seeded.");
}
