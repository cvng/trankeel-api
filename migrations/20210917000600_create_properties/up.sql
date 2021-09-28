CREATE TYPE propertystatus AS ENUM ('for_sale', 'inactive', 'rented', 'under_construction', 'unrented');

CREATE TABLE properties (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    address JSONB NOT NULL,
    build_period TEXT,
    building_legal_status TEXT,
    common_spaces TEXT,
    energy_class TEXT,
    equipments TEXT,
    gas_emission TEXT,
    heating_method TEXT,
    housing_type TEXT,
    name TEXT NOT NULL,
    note TEXT,
    description TEXT,
    ntic_equipments TEXT,
    other_spaces TEXT,
    tax NUMERIC,
    room_count TEXT NOT NULL,
    status PROPERTYSTATUS NOT NULL,
    surface REAL NOT NULL,
    tenant_private_spaces TEXT,
    usage_type TEXT,
    water_heating_method TEXT,
    lender_id UUID NOT NULL REFERENCES lenders(id)
);

SELECT manage_updated_at('properties');
