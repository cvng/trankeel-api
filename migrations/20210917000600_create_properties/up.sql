CREATE TYPE propertystatus AS ENUM (
    'for_sale',
    'inactive',
    'rented',
    'under_construction',
    'unrented'
);

CREATE TYPE propertyroomtype AS ENUM (
    'other',
    't1',
    't2',
    't3',
    't4',
    't5',
    't6'
);

CREATE TYPE propertybuildperiodtype AS ENUM (
    'before_1949',
    'from_1949_1974',
    'from_1975_1989',
    'from_1990_2005',
    'from_2005'
);

CREATE TYPE propertyenergyclass AS ENUM (
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g'
);

CREATE TYPE propertygasemission AS ENUM (
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g'
);

CREATE TYPE propertybuildinglegalstatus AS ENUM (
    'copro',
    'mono'
);

CREATE TYPE propertyhabitationusagetype AS ENUM (
    'habitation',
    'mixte'
);

CREATE TYPE propertyusagetype AS ENUM (
    'collective',
    'individual'
);

CREATE TABLE properties (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ,
    account_id UUID NOT NULL REFERENCES accounts(id),
    address JSONB NOT NULL,
    build_period PROPERTYBUILDPERIODTYPE,
    building_legal_status PROPERTYBUILDINGLEGALSTATUS,
    common_spaces TEXT,
    energy_class PROPERTYENERGYCLASS,
    equipments TEXT,
    gas_emission PROPERTYGASEMISSION,
    heating_method PROPERTYUSAGETYPE,
    housing_type PROPERTYUSAGETYPE,
    name TEXT NOT NULL,
    note TEXT,
    ntic_equipments TEXT,
    other_spaces TEXT,
    tax NUMERIC,
    room_count PROPERTYROOMTYPE,
    status PROPERTYSTATUS NOT NULL,
    surface REAL,
    tenant_private_spaces TEXT,
    usage_type PROPERTYHABITATIONUSAGETYPE,
    water_heating_method PROPERTYUSAGETYPE,
    lender_id UUID NOT NULL REFERENCES lenders(id)
);

SELECT manage_updated_at('properties');
