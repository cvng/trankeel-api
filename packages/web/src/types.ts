import { TypedDocumentNode as DocumentNode } from "@graphql-typed-document-node/core";
export type Maybe<T> = T | null;
export type Exact<T extends { [key: string]: unknown }> = {
  [K in keyof T]: T[K];
};
export type MakeOptional<T, K extends keyof T> = Omit<T, K> &
  { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> &
  { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export interface Scalars {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
}

export interface Address {
  __typename?: "Address";
  city: Scalars["String"];
  line1: Scalars["String"];
  line2?: Maybe<Scalars["String"]>;
  postalCode: Scalars["String"];
}

export interface Mutation {
  __typename?: "Mutation";
  userCreateWithAccount: Scalars["String"];
}

export interface Person {
  __typename?: "Person";
  authId: Scalars["String"];
  email: Scalars["String"];
  firstName?: Maybe<Scalars["String"]>;
  lastName?: Maybe<Scalars["String"]>;
  photoUrl?: Maybe<Scalars["String"]>;
  role?: Maybe<Scalars["String"]>;
  id: Scalars["String"];
  phoneNumber?: Maybe<Scalars["String"]>;
  accountId?: Maybe<Scalars["String"]>;
}

export interface Property {
  __typename?: "Property";
  accountId?: Maybe<Scalars["String"]>;
  address: Address;
  buildPeriod?: Maybe<Scalars["String"]>;
  buildingLegalStatus?: Maybe<Scalars["String"]>;
  commonSpaces?: Maybe<Scalars["String"]>;
  energyClass?: Maybe<Scalars["String"]>;
  equipments?: Maybe<Scalars["String"]>;
  gasEmission?: Maybe<Scalars["String"]>;
  heatingMethod?: Maybe<Scalars["String"]>;
  housingType?: Maybe<Scalars["String"]>;
  name: Scalars["String"];
  note?: Maybe<Scalars["String"]>;
  nticEquipments?: Maybe<Scalars["String"]>;
  otherSpaces?: Maybe<Scalars["String"]>;
  tax?: Maybe<Scalars["Float"]>;
  roomCount: Scalars["String"];
  status?: Maybe<Scalars["String"]>;
  surface: Scalars["Int"];
  tenantPrivateSpaces?: Maybe<Scalars["String"]>;
  usageType?: Maybe<Scalars["String"]>;
  waterHeatingMethod?: Maybe<Scalars["String"]>;
  id: Scalars["String"];
  lenderId: Scalars["String"];
}

export interface Query {
  __typename?: "Query";
  viewer: Person;
  properties: Array<Property>;
}

/** A GraphQL Schema defines the capabilities of a GraphQL server. It exposes all available types and directives on the server, as well as the entry points for query, mutation, and subscription operations. */
export interface __Schema {
  __typename?: "__Schema";
  description?: Maybe<Scalars["String"]>;
  /** A list of all types supported by this server. */
  types: Array<__Type>;
  /** The type that query operations will be rooted at. */
  queryType: __Type;
  /** If this server supports mutation, the type that mutation operations will be rooted at. */
  mutationType?: Maybe<__Type>;
  /** If this server support subscription, the type that subscription operations will be rooted at. */
  subscriptionType?: Maybe<__Type>;
  /** A list of all directives supported by this server. */
  directives: Array<__Directive>;
}

/**
 * The fundamental unit of any GraphQL Schema is the type. There are many kinds of types in GraphQL as represented by the `__TypeKind` enum.
 *
 * Depending on the kind of a type, certain fields describe information about that type. Scalar types provide no information beyond a name, description and optional `specifiedByUrl`, while Enum types provide their values. Object and Interface types provide the fields they describe. Abstract types, Union and Interface, provide the Object types possible at runtime. List and NonNull types compose other types.
 */
export interface __Type {
  __typename?: "__Type";
  kind: __TypeKind;
  name?: Maybe<Scalars["String"]>;
  description?: Maybe<Scalars["String"]>;
  specifiedByUrl?: Maybe<Scalars["String"]>;
  fields?: Maybe<Array<__Field>>;
  interfaces?: Maybe<Array<__Type>>;
  possibleTypes?: Maybe<Array<__Type>>;
  enumValues?: Maybe<Array<__EnumValue>>;
  inputFields?: Maybe<Array<__InputValue>>;
  ofType?: Maybe<__Type>;
}

/**
 * The fundamental unit of any GraphQL Schema is the type. There are many kinds of types in GraphQL as represented by the `__TypeKind` enum.
 *
 * Depending on the kind of a type, certain fields describe information about that type. Scalar types provide no information beyond a name, description and optional `specifiedByUrl`, while Enum types provide their values. Object and Interface types provide the fields they describe. Abstract types, Union and Interface, provide the Object types possible at runtime. List and NonNull types compose other types.
 */
export interface __TypeFieldsArgs {
  includeDeprecated?: Maybe<Scalars["Boolean"]>;
}

/**
 * The fundamental unit of any GraphQL Schema is the type. There are many kinds of types in GraphQL as represented by the `__TypeKind` enum.
 *
 * Depending on the kind of a type, certain fields describe information about that type. Scalar types provide no information beyond a name, description and optional `specifiedByUrl`, while Enum types provide their values. Object and Interface types provide the fields they describe. Abstract types, Union and Interface, provide the Object types possible at runtime. List and NonNull types compose other types.
 */
export interface __TypeEnumValuesArgs {
  includeDeprecated?: Maybe<Scalars["Boolean"]>;
}

/**
 * The fundamental unit of any GraphQL Schema is the type. There are many kinds of types in GraphQL as represented by the `__TypeKind` enum.
 *
 * Depending on the kind of a type, certain fields describe information about that type. Scalar types provide no information beyond a name, description and optional `specifiedByUrl`, while Enum types provide their values. Object and Interface types provide the fields they describe. Abstract types, Union and Interface, provide the Object types possible at runtime. List and NonNull types compose other types.
 */
export interface __TypeInputFieldsArgs {
  includeDeprecated?: Maybe<Scalars["Boolean"]>;
}

/** An enum describing what kind of type a given `__Type` is. */
export enum __TypeKind {
  /** Indicates this type is a scalar. */
  Scalar = "SCALAR",
  /** Indicates this type is an object. `fields` and `interfaces` are valid fields. */
  Object = "OBJECT",
  /** Indicates this type is an interface. `fields`, `interfaces`, and `possibleTypes` are valid fields. */
  Interface = "INTERFACE",
  /** Indicates this type is a union. `possibleTypes` is a valid field. */
  Union = "UNION",
  /** Indicates this type is an enum. `enumValues` is a valid field. */
  Enum = "ENUM",
  /** Indicates this type is an input object. `inputFields` is a valid field. */
  InputObject = "INPUT_OBJECT",
  /** Indicates this type is a list. `ofType` is a valid field. */
  List = "LIST",
  /** Indicates this type is a non-null. `ofType` is a valid field. */
  NonNull = "NON_NULL",
}

/** Object and Interface types are described by a list of Fields, each of which has a name, potentially a list of arguments, and a return type. */
export interface __Field {
  __typename?: "__Field";
  name: Scalars["String"];
  description?: Maybe<Scalars["String"]>;
  args: Array<__InputValue>;
  type: __Type;
  isDeprecated: Scalars["Boolean"];
  deprecationReason?: Maybe<Scalars["String"]>;
}

/** Object and Interface types are described by a list of Fields, each of which has a name, potentially a list of arguments, and a return type. */
export interface __FieldArgsArgs {
  includeDeprecated?: Maybe<Scalars["Boolean"]>;
}

/** Arguments provided to Fields or Directives and the input fields of an InputObject are represented as Input Values which describe their type and optionally a default value. */
export interface __InputValue {
  __typename?: "__InputValue";
  name: Scalars["String"];
  description?: Maybe<Scalars["String"]>;
  type: __Type;
  /** A GraphQL-formatted string representing the default value for this input value. */
  defaultValue?: Maybe<Scalars["String"]>;
  isDeprecated: Scalars["Boolean"];
  deprecationReason?: Maybe<Scalars["String"]>;
}

/** One possible value for a given Enum. Enum values are unique values, not a placeholder for a string or numeric value. However an Enum value is returned in a JSON response as a string. */
export interface __EnumValue {
  __typename?: "__EnumValue";
  name: Scalars["String"];
  description?: Maybe<Scalars["String"]>;
  isDeprecated: Scalars["Boolean"];
  deprecationReason?: Maybe<Scalars["String"]>;
}

/**
 * A Directive provides a way to describe alternate runtime execution and type validation behavior in a GraphQL document.
 *
 * In some cases, you need to provide options to alter GraphQL's execution behavior in ways field arguments will not suffice, such as conditionally including or skipping a field. Directives provide this by describing additional information to the executor.
 */
export interface __Directive {
  __typename?: "__Directive";
  name: Scalars["String"];
  description?: Maybe<Scalars["String"]>;
  isRepeatable: Scalars["Boolean"];
  locations: Array<__DirectiveLocation>;
  args: Array<__InputValue>;
}

/** A Directive can be adjacent to many parts of the GraphQL language, a __DirectiveLocation describes one such possible adjacencies. */
export enum __DirectiveLocation {
  /** Location adjacent to a query operation. */
  Query = "QUERY",
  /** Location adjacent to a mutation operation. */
  Mutation = "MUTATION",
  /** Location adjacent to a subscription operation. */
  Subscription = "SUBSCRIPTION",
  /** Location adjacent to a field. */
  Field = "FIELD",
  /** Location adjacent to a fragment definition. */
  FragmentDefinition = "FRAGMENT_DEFINITION",
  /** Location adjacent to a fragment spread. */
  FragmentSpread = "FRAGMENT_SPREAD",
  /** Location adjacent to an inline fragment. */
  InlineFragment = "INLINE_FRAGMENT",
  /** Location adjacent to a variable definition. */
  VariableDefinition = "VARIABLE_DEFINITION",
  /** Location adjacent to a schema definition. */
  Schema = "SCHEMA",
  /** Location adjacent to a scalar definition. */
  Scalar = "SCALAR",
  /** Location adjacent to an object type definition. */
  Object = "OBJECT",
  /** Location adjacent to a field definition. */
  FieldDefinition = "FIELD_DEFINITION",
  /** Location adjacent to an argument definition. */
  ArgumentDefinition = "ARGUMENT_DEFINITION",
  /** Location adjacent to an interface definition. */
  Interface = "INTERFACE",
  /** Location adjacent to a union definition. */
  Union = "UNION",
  /** Location adjacent to an enum definition. */
  Enum = "ENUM",
  /** Location adjacent to an enum value definition. */
  EnumValue = "ENUM_VALUE",
  /** Location adjacent to an input object type definition. */
  InputObject = "INPUT_OBJECT",
  /** Location adjacent to an input object field definition. */
  InputFieldDefinition = "INPUT_FIELD_DEFINITION",
}

export type TestQueryVariables = Exact<{ [key: string]: never }>;

export type TestQuery = { __typename?: "Query" } & {
  __schema: { __typename?: "__Schema" } & {
    types: Array<{ __typename?: "__Type" } & Pick<__Type, "name">>;
  };
};

export type UserQueryVariables = Exact<{ [key: string]: never }>;

export type UserQuery = { __typename?: "Query" } & {
  user: { __typename?: "Person" } & Pick<
    Person,
    "id" | "email" | "firstName" | "lastName"
  >;
};

export type PropertyListQueryVariables = Exact<{ [key: string]: never }>;

export type PropertyListQuery = { __typename?: "Query" } & {
  properties: Array<
    { __typename?: "Property" } & Pick<
      Property,
      | "id"
      | "name"
      | "roomCount"
      | "note"
      | "surface"
      | "status"
      | "energyClass"
      | "gasEmission"
      | "tenantPrivateSpaces"
      | "equipments"
      | "nticEquipments"
      | "otherSpaces"
      | "commonSpaces"
      | "waterHeatingMethod"
      | "heatingMethod"
      | "tax"
      | "buildPeriod"
      | "housingType"
      | "buildingLegalStatus"
      | "usageType"
    > & {
        address: { __typename?: "Address" } & Pick<
          Address,
          "city" | "line1" | "line2" | "postalCode"
        >;
      }
  >;
};

export const TestDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "Test" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "__schema" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  name: { kind: "Name", value: "types" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<TestQuery, TestQueryVariables>;
export const UserDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "User" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            alias: { kind: "Name", value: "user" },
            name: { kind: "Name", value: "viewer" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "email" } },
                { kind: "Field", name: { kind: "Name", value: "firstName" } },
                { kind: "Field", name: { kind: "Name", value: "lastName" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<UserQuery, UserQueryVariables>;
export const PropertyListDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "PropertyList" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "properties" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "name" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "address" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "city" } },
                      { kind: "Field", name: { kind: "Name", value: "line1" } },
                      { kind: "Field", name: { kind: "Name", value: "line2" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "postalCode" },
                      },
                    ],
                  },
                },
                { kind: "Field", name: { kind: "Name", value: "roomCount" } },
                { kind: "Field", name: { kind: "Name", value: "note" } },
                { kind: "Field", name: { kind: "Name", value: "surface" } },
                { kind: "Field", name: { kind: "Name", value: "status" } },
                { kind: "Field", name: { kind: "Name", value: "energyClass" } },
                { kind: "Field", name: { kind: "Name", value: "gasEmission" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "tenantPrivateSpaces" },
                },
                { kind: "Field", name: { kind: "Name", value: "equipments" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "nticEquipments" },
                },
                { kind: "Field", name: { kind: "Name", value: "otherSpaces" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "commonSpaces" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "waterHeatingMethod" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "heatingMethod" },
                },
                { kind: "Field", name: { kind: "Name", value: "tax" } },
                { kind: "Field", name: { kind: "Name", value: "buildPeriod" } },
                { kind: "Field", name: { kind: "Name", value: "housingType" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "buildingLegalStatus" },
                },
                { kind: "Field", name: { kind: "Name", value: "usageType" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<PropertyListQuery, PropertyListQueryVariables>;
