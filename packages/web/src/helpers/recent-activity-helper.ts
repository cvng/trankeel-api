import moment from "moment";
import { translate } from "piteo-kit";
import { EventModel } from "../components/recent-activities/recent-activity-item";
import { Event, EventType, Tenant } from "../types";
import { NumberHelper } from "./number-helper";

const _ = translate();

const titleForEvent = (event: Event): string => {
  switch (event.type) {
    case EventType.RentReceiptCreated:
    case EventType.TransactionCreated:
      return ""; // No title for these types
    case EventType.RentReceiptSent:
      return _("rent_receipt_sent_title");
    default:
      return "";
  }
};

const contentForEvent = (event: Event): string => {
  const participants = event?.object?.lease?.tenants?.map((tenant: Tenant) =>
    tenant.displayName
  ).join(` ${_("and")} `);
  switch (event.type) {
    case EventType.RentReceiptCreated:
      return _("rent_receipt_created", { tenantName: participants });
    case EventType.TransactionCreated:
      return _("transaction_created", {
        tenantName: participants,
        amount: NumberHelper.formatToString(event?.object?.amount, false),
      });
    case EventType.RentReceiptSent:
      return _("rent_receipt_sent", { tenantName: participants });
    default:
      return "";
  }
};

const readableDateForEvent = (event: Event): string => {
  return moment(event.createdAt).startOf("minutes").fromNow();
};

export const eventModel = (event: Event): EventModel => {
  return {
    createdAt: readableDateForEvent(event),
    type: event.type,
    content: contentForEvent(event),
    flag: { title: titleForEvent(event) },
    user: event?.object?.lease?.tenants?.[0],
  };
};
