use chrono::Utc;
use trankeel::AddressInput;
use trankeel::Amount;
use trankeel::AuthId;
use trankeel::CreateAdvertisementInput;
use trankeel::CreateCandidacyInput;
use trankeel::CreateFurnishedLeaseInput;
use trankeel::CreateProfessionalWarrantInput;
use trankeel::CreatePropertyInput;
use trankeel::CreateTenantInput;
use trankeel::CreateUserWithAccountInput;
use trankeel::CreateWarrantInput;
use trankeel::EntryFlexibility;
use trankeel::LeaseType;
use trankeel::WarrantType;

pub async fn seed() {
    let config = trankeel::config::config();
    let client = trankeel::init(&config).unwrap();

    let auth_id = AuthId::new(config.debug_auth_id.clone().unwrap());
    let author = config.author();

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
        .unwrap()
        .user;

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
                build_period: None,
                building_legal_status: None,
                common_spaces: None,
                energy_class: None,
                equipments: None,
                gas_emission: None,
                heating_method: None,
                housing_type: None,
                lender_id: None,
                name: "Petite mission".into(),
                note: Some("RAS".into()),
                description: Some("Description".into()),
                ntic_equipments: None,
                other_spaces: None,
                room_count: None,
                status: None,
                surface: None,
                tax: None,
                tenant_private_spaces: None,
                usage_type: None,
                water_heating_method: None,
            },
        )
        .await
        .unwrap();

    let tenant = client
        .create_tenant(
            &auth_id,
            CreateTenantInput {
                birthdate: Some(Utc::now().date().naive_utc().into()),
                birthplace: None,
                email: author.email.clone(),
                first_name: "Tenant".into(),
                last_name: "TRANKEEL".into(),
                note: None,
                phone_number: None,
                is_student: Some(false),
                warrants: None,
            },
        )
        .await
        .unwrap();

    let lease = client
        .create_furnished_lease(
            &auth_id,
            CreateFurnishedLeaseInput {
                details: None,
                deposit_amount: Amount::new(36000),
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
        .await
        .unwrap();

    let advertisement = client
        .create_advertisement(
            &auth_id,
            CreateAdvertisementInput {
                published: true,
                lease_type: LeaseType::Furnished,
                rent_amount: Amount::new(36000),
                rent_charges_amount: Some(Amount::new(9000)),
                deposit_amount: Amount::new(36000),
                effect_date: Utc::now().into(),
                flexibility: Some(EntryFlexibility::OneDay),
                referral_lease_id: Some(lease.id),
                property_id: property.id,
                title: "Title".into(),
                description: "Description".into(),
            },
        )
        .await
        .unwrap();

    let candidacy = client
        .create_candidacy(CreateCandidacyInput {
            advertisement_id: advertisement.id,
            email: author.email,
            first_name: "Candidate".into(),
            last_name: "TRANKEEL".into(),
            phone_number: "+33633123456".to_string().into(),
            move_in_date: Utc::now().into(),
            description: "Hello, Lender!".into(),
            birthdate: Utc::now().date().naive_utc().into(),
            birthplace: None,
            is_student: true,
            files: None,
            warrants: Some(vec![CreateWarrantInput {
                type_: WarrantType::Visale,
                individual: None,
                company: Some(CreateProfessionalWarrantInput {
                    name: "ABC".into(),
                    identifier: "001".into(),
                }),
            }]),
        })
        .await
        .unwrap();

    println!(
        "{:#?}\n{:#?}\n{:#?}\n{:#?}\n{:#?}\n{:#?}\n{:#?}",
        user, lender, property, tenant, lease, advertisement, candidacy
    );
    println!("🌱 Database seeded.");
}
