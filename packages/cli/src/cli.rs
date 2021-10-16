mod seed_util;

use crate::seed_util::author;
use chrono::Utc;
use piteo::AddressInput;
use piteo::Amount;
use piteo::AuthId;
use piteo::CreateAdvertisementInput;
use piteo::CreateCandidacyInput;
use piteo::CreateFurnishedLeaseInput;
use piteo::CreatePropertyInput;
use piteo::CreateTenantInput;
use piteo::CreateUserWithAccountInput;
use piteo::EntryFlexibility;
use piteo::LeaseType;
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
    let client = piteo::init().unwrap();

    let auth_id = AuthId::new(env::var("DEBUG_AUTH_ID").unwrap());
    let author = author(env::var("AUTHOR").unwrap()).unwrap();

    let user = client
        .create_user_with_account(CreateUserWithAccountInput {
            auth_id: auth_id.clone(),
            email: author.email.clone(),
            first_name: author.first_name,
            last_name: author.last_name,
            address: Some(AddressInput {
                city: "PTP".into(),
                line1: "542".into(),
                postal_code: "97110".into(),
                country: None,
                line2: None,
            }),
            phone_number: None,
            skip_create_customer: Some(true),
        })
        .await
        .unwrap();

    let (lender, _) = client.lenders().by_individual_id(&user.id).unwrap();

    let property = client
        .create_property(
            &auth_id,
            CreatePropertyInput {
                address: AddressInput {
                    city: "Talence".into(),
                    line1: "27 Rue de la petite mission".into(),
                    line2: Some("Etg 1 apt 12".into()),
                    postal_code: "16000".into(),
                    country: Default::default(),
                },
                build_period: PropertyBuildPeriodType::From_1949_1974,
                building_legal_status: PropertyBuildingLegalStatus::Copro,
                common_spaces: None,
                energy_class: None,
                equipments: None,
                gas_emission: None,
                heating_method: PropertyUsageType::Collective,
                housing_type: PropertyUsageType::Individual,
                lender_id: lender.id,
                name: "Petite mission".into(),
                note: Some("RAS".into()),
                description: Some("Description".into()),
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

    let tenant = client
        .create_tenant(
            &auth_id,
            CreateTenantInput {
                apl: None,
                birthdate: Some(Utc::now().date().naive_utc().into()),
                birthplace: None,
                email: author.email.clone(),
                first_name: "Tenant".into(),
                last_name: "PITEO".into(),
                note: None,
                phone_number: None,
                is_student: Some(false),
                warrants: None,
            },
        )
        .unwrap();

    let lease = client
        .create_furnished_lease(
            &auth_id,
            CreateFurnishedLeaseInput {
                details: None,
                deposit_amount: None,
                effect_date: Utc::now().into(),
                renew_date: None,
                file: None,
                property_id: property.id,
                rent_amount: Amount::new(36000),
                rent_charges_amount: Some(Amount::new(9000)),
                signature_date: None,
                tenant_ids: vec![tenant.id],
            },
        )
        .unwrap();

    let advertisement = client
        .create_advertisement(
            &auth_id,
            CreateAdvertisementInput {
                published: true,
                lease_type: LeaseType::Furnished,
                rent_amount: Amount::new(36000),
                rent_charges_amount: Some(Amount::new(9000)),
                deposit_amount: None,
                effect_date: Utc::now().into(),
                flexibility: Some(EntryFlexibility::OneDay),
                referral_lease_id: Some(lease.id),
                property_id: property.id,
                title: "Title".into(),
                description: "Description".into(),
            },
        )
        .unwrap();

    let candidacy = client
        .create_candidacy(CreateCandidacyInput {
            advertisement_id: advertisement.id,
            is_student: true,
            first_name: "Candidate".into(),
            last_name: "PITEO".into(),
            birthdate: Utc::now().date().naive_utc().into(),
            email: author.email,
            phone_number: "+33633123456".to_string().into(),
            move_in_date: Utc::now().into(),
            description: "Hello, Lender!".into(),
            files: None,
            warrants: None,
        })
        .await
        .unwrap();

    println!(
        "{:#?}\n{:#?}\n{:#?}\n{:#?}\n{:#?}\n{:#?}\n{:#?}",
        user, lender, property, tenant, lease, advertisement, candidacy
    );
    println!("🌱 Database seeded.");
}
