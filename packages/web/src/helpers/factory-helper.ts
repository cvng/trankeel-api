// @ts-nocheck
import {
  Account,
  Address,
  Event,
  EventType,
  Feature,
  File,
  FileStatus,
  FileType,
  Lease,
  LeaseFurnishedDuration,
  LeaseStatus,
  LeaseType,
  LegalEntityType,
  Lender,
  Plan,
  PlanCode,
  Property,
  PropertyBuildingLegalStatus,
  PropertyBuildPeriodType,
  PropertyEnergyClass,
  PropertyGasEmission,
  PropertyHabitationUsageType,
  PropertyRoomType,
  PropertyStatus,
  PropertyUsageType,
  Rent,
  RentPaymentMethod,
  RentStatus,
  SubscriptionStatus,
  Tenant,
  TenantStatus,
} from "../types";

export class FactoryHelper {
  // Leases
  static leaseList(): Lease[] {
    return [
      {
        account: this.account(),
        accountId: "001",
        effectDate: new Date().toLocaleString("fr-FR"),
        renewDate: new Date().toLocaleString("fr-FR"),
        depositAmount: 370,
        rentAmount: 370,
        rentChargesAmount: 80,
        data: {
          rentPaymentMethod: RentPaymentMethod.Before,
          duration: LeaseFurnishedDuration.OneYear,
        },
        lease: undefined,
        id: "001",
        status: LeaseStatus.Active,
        property: {
          account: this.account(),
          accountId: "",
          id: "03",
          name: "NOTRE DAME",
          roomCount: PropertyRoomType.T6,
          surface: 100,
          status: PropertyStatus.Rented,
          address: this.address(),
          leases: [],
          housingType: PropertyUsageType.Collective,
          buildPeriod: PropertyBuildPeriodType.BeforeY1949,
          usageType: PropertyHabitationUsageType.Habitation,
          buildingLegalStatus: PropertyBuildingLegalStatus.Copro,
          tax: 620,
          heatingMethod: PropertyUsageType.Collective,
          waterHeatingMethod: PropertyUsageType.Collective,
        },
        propertyId: "002",
        rentFullAmount: 450,
        tenants: this.tenantList(),
        type: LeaseType.Furnished,
      },
    ];
  }

  // Features
  static featureList(): Feature[] {
    return [
      {
        available: true,
        key: "BOOSTER_KEY",
        title: "Booster 🚀",
      },
    ];
  }

  // Plans
  static plan(): Plan {
    return {
      code: PlanCode.Solo,
      features: this.featureList(),
      id: "001",
      price: 990,
      subtitle: "Idéal pour les investisseurs",
      title: "Plan SOLO",
    };
  }

  // Address
  static address(): Address {
    return {
      city: "Bordeaux",
      country: "FRANCE",
      inline: "12 Rue du Palais Galien, 2ème étage BAT C, 33000 Bordeaux",
      line1: "12 Rue du Palais Galien",
      line2: "2ème étage BAT C",
      postalCode: "33000",
    };
  }

  // Account
  static account(): Account {
    return {
      id: "0033",
      plan: this.plan(),
      status: SubscriptionStatus.Active,
      stripeCustomerId: "",
      stripeSubscriptionId: "",
      trialEnd: undefined,
    };
  }

  // Tenants
  static tenantList(): Tenant[] {
    return [
      {
        account: this.account(),
        accountId: "",
        id: "01",
        firstName: "Lenie",
        lastName: "Carpentro",
        displayName: "Lenie Carpentro",
        email: "support@piteo.fr",
        phoneNumber: "0657898790",
        fullName: "Lenie Carpentro",
        status: TenantStatus.New,
        note: "Doit envoyer son justificatif d'assurance",
        apl: true,
        visaleId: "",
        birthdate: null,
        birthplace: "Pointe-à-Pitre",
        propertyName: "JULES FERRY",
        lease: {
          id: "001",
          account: this.account(),
          accountId: "",
          rentFullAmount: 365,
          propertyId: "0023",
          property: null,
          tenants: [],
          effectDate: new Date().toLocaleString("fr-FR"),
          renewDate: new Date().toLocaleString("fr-FR"),
          rentAmount: 315,
          rentChargesAmount: 50,
          data: {
            duration: LeaseFurnishedDuration.OneYear,
          },
          type: LeaseType.Furnished,
          status: LeaseStatus.Active,
        },
      },
      {
        account: this.account(),
        accountId: "",
        id: "02",
        firstName: "Jean-Michel",
        lastName: "Dupuy",
        displayName: "Jean-Michel Dupuy",
        email: "support@piteo.fr",
        phoneNumber: "0657898790",
        fullName: "Jean-Michel Dupuy",
        status: TenantStatus.Late,
        note: "Doit envoyer son justificatif de passport VISALE",
        apl: false,
        visaleId: "V28989182",
        birthdate: null,
        birthplace: "Bordeaux",
        propertyName: "MONTMOREAU",
        lease: {
          id: "001",
          account: this.account(),
          accountId: "",
          rentFullAmount: 450,
          propertyId: "0022",
          property: null,
          tenants: [],
          effectDate: new Date().toLocaleString("fr-FR"),
          renewDate: new Date().toLocaleString("fr-FR"),
          rentAmount: 370,
          rentChargesAmount: 80,
          depositAmount: 315,
          data: {
            duration: LeaseFurnishedDuration.OneYear,
          },
          type: LeaseType.Furnished,
          status: LeaseStatus.Active,
        },
      },
    ];
  }

  // Properties
  static propertyList(): Property[] {
    return [{
      id: "03",
      name: "NOTRE DAME R+2",
      roomCount: PropertyRoomType.T1,
      surface: 27,
      status: PropertyStatus.Rented,
      address: this.address(),
      leases: this.leaseList(),
      housingType: PropertyUsageType.Collective,
      buildPeriod: PropertyBuildPeriodType.FromY1975Y1989,
      usageType: PropertyHabitationUsageType.Habitation,
      gasEmission: PropertyGasEmission.C,
      energyClass: PropertyEnergyClass.B,
      buildingLegalStatus: PropertyBuildingLegalStatus.Copro,
      heatingMethod: PropertyUsageType.Collective,
      waterHeatingMethod: PropertyUsageType.Collective,
    }, {
      id: "04",
      name: "JULES FERRY",
      address: this.address(),
      surface: 23,
      roomCount: PropertyRoomType.T1,
      status: PropertyStatus.Rented,
      collectedRents: 930,
      expectedRents: 930,
      leases: this.leaseList(),
      accountId: "",
      housingType: PropertyUsageType.Individual,
      buildPeriod: PropertyBuildPeriodType.BeforeY1949,
      usageType: PropertyHabitationUsageType.Habitation,
      buildingLegalStatus: PropertyBuildingLegalStatus.Mono,
      heatingMethod: PropertyUsageType.Collective,
      waterHeatingMethod: PropertyUsageType.Collective,
    }, {
      id: "05",
      name: "MONTMOREAU",
      address: this.address(),
      surface: 23,
      roomCount: PropertyRoomType.T1,
      status: PropertyStatus.Rented,
      collectedRents: 450,
      expectedRents: 450,
      leases: this.leaseList(),
      accountId: "",
      housingType: PropertyUsageType.Individual,
      buildPeriod: PropertyBuildPeriodType.BeforeY1949,
      usageType: PropertyHabitationUsageType.Habitation,
      buildingLegalStatus: PropertyBuildingLegalStatus.Copro,
      heatingMethod: PropertyUsageType.Collective,
      waterHeatingMethod: PropertyUsageType.Collective,
    }, {
      id: "06",
      name: "SIMARD",
      address: this.address(),
      surface: 56,
      roomCount: PropertyRoomType.T1,
      status: PropertyStatus.Rented,
      collectedRents: 450,
      expectedRents: 450,
      leases: [],
      accountId: "",
      housingType: PropertyUsageType.Individual,
      buildPeriod: PropertyBuildPeriodType.BeforeY1949,
      usageType: PropertyHabitationUsageType.Mixte,
      buildingLegalStatus: PropertyBuildingLegalStatus.Mono,
      heatingMethod: PropertyUsageType.Collective,
      waterHeatingMethod: PropertyUsageType.Collective,
    }];
  }

  // Identities
  static lenderList(): Lender[] {
    return [
      {
        id: "001",
        accountId: "00000",
        displayName: "SCI PITEO",
        identity: {
          id: "001a",
          email: "gail@piteo.fr",
          phoneNumber: "06 34 82 92 02",
          legalEntity: "SCI PITEO",
          legalEntityIdentifier: "530 706 738 00040",
          legalEntityType: LegalEntityType.Sci,
          displayName: "SCI PITEO",
          address: this.address(),
          __typename: "Company",
        },
      },
      {
        id: "002",
        accountId: "00000",
        displayName: "Gaïl SANCTUSSY",
        identity: {
          id: "002a",
          email: "gail@piteo.fr",
          phoneNumber: "06 34 82 92 02",
          firstName: "Gaïl",
          lastName: "SANCTUSSY",
          displayName: "Gaïl SANCTUSSY",
          __typename: "User",
        },
      },
    ];
  }

  // Files
  static fileList(): File[] {
    return [
      {
        createdAt: new Date().toLocaleString("fr-FR"),
        downloadUrl: "http://www.africau.edu/images/default/sample.pdf",
        externalId: "fakeId",
        filename: "Quittance (juin 2021)",
        id: "0001",
        previewUrl: "http://www.africau.edu/images/default/sample.pdf",
        status: FileStatus.Success,
        type: FileType.RentReceipt,
        updatedAt: new Date().toLocaleString("fr-FR"),
      },
      {
        createdAt: new Date().toLocaleString("fr-FR"),
        downloadUrl: "http://www.africau.edu/images/default/sample.pdf",
        externalId: "fakeId",
        filename: "Quittance (mai 2021)",
        id: "0002",
        previewUrl: "http://www.africau.edu/images/default/sample.pdf",
        status: FileStatus.Success,
        type: FileType.RentReceipt,
        updatedAt: new Date().toLocaleString("fr-FR"),
      },
      {
        createdAt: new Date().toLocaleString("fr-FR"),
        downloadUrl: "http://www.africau.edu/images/default/sample.pdf",
        externalId: "fakeId",
        filename: "Quittance (avril 2021)",
        id: "0002",
        previewUrl: "http://www.africau.edu/images/default/sample.pdf",
        status: FileStatus.Success,
        type: FileType.RentReceipt,
        updatedAt: new Date().toLocaleString("fr-FR"),
      },
      {
        createdAt: new Date().toLocaleString("fr-FR"),
        downloadUrl: "http://www.africau.edu/images/default/sample.pdf",
        externalId: "fakeId",
        filename: "Avis d'échéance (juin 2021)",
        id: "0001",
        previewUrl: "http://www.africau.edu/images/default/sample.pdf",
        status: FileStatus.Success,
        type: FileType.PaymentNotice,
        updatedAt: new Date().toLocaleString("fr-FR"),
      },
      {
        createdAt: new Date().toLocaleString("fr-FR"),
        downloadUrl: "http://www.africau.edu/images/default/sample.pdf",
        externalId: "fakeId",
        filename: "Bail",
        id: "0001",
        previewUrl: "http://www.africau.edu/images/default/sample.pdf",
        status: FileStatus.Success,
        type: FileType.LeaseDocument,
        updatedAt: new Date().toLocaleString("fr-FR"),
      },
    ];
  }

  // Rent
  static rentList(): Rent[] {
    return [
      {
        status: RentStatus.Pending,
        amount: 330,
        chargesAmount: 50,
        fullAmount: 380,
        delay: 2,
        lease: this.leaseList()?.[0],
        leaseId: null,
        id: "rentId0",
        periodEnd: "2021-07-31T00:00:00.000Z",
        periodStart: "2021-08-31T00:00:00.000Z",
        receipt: null,
        receiptId: null,
        transactions: [],
      },
    ];
  }

  // Event
  static eventList(): Event[] {
    return [
      {
        eventableId: "",
        eventableType: "",
        createdAt: new Date("2021-09-02").toISOString(),
        id: "",
        object: this.rentList()[0],
        type: EventType.RentReceiptSent,
      },
      {
        eventableId: "",
        eventableType: "",
        createdAt: new Date("2021-09-08").toISOString(),
        id: "",
        object: this.rentList()[0],
        type: EventType.RentReceiptCreated,
      },
      {
        eventableId: "",
        eventableType: "",
        createdAt: new Date().toISOString(),
        id: "",
        object: this.rentList()[0],
        type: EventType.TransactionCreated,
      },
    ];
  }
}
