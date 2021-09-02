import { translate } from "piteo-kit";
import { FunctionComponent, useContext, useState } from "react";
import { useRouter } from "../../hooks/use-router";
import { Document } from "../document/document";
import { withContainer as LeaseFurnishedContainer } from "../document/document-lease-furnished-container";
import {
  LeaseAddExistingFormContainer,
  LeaseExistingForm,
} from "../lease-existing-form";
import {
  LeaseSelectParties,
  LeaseSelectPartiesContainer,
} from "../lease-select-parties";
import { LeaseAddConfirmation } from "./lease-add-confirmation";
import { LeaseAddContext, LeaseAddContextProps } from "./lease-add-context";
import {
  LeaseAddFlowDelegate,
  LeaseAddFlowPageProps,
} from "./lease-add-flow-page";

const _ = translate();

export type WrappedComponentProps = FunctionComponent<LeaseAddFlowPageProps>;

export const withContainer = (
  WrappedComponent: WrappedComponentProps,
): FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const [selectedIndex, setSelectedIndex] = useState(0);
    const [checkedItems, setCheckedItems] = useState([]);

    const {
      createFromExistingLease,
      finishFlowWithError,
      selectedProperty,
    } = useContext(
      LeaseAddContext,
    ) as LeaseAddContextProps;

    const menuItemClick = (index: number) => {
      // This function is called only is the menu item is enabled,
      // so we can just change the selected index
      setSelectedIndex(index);
    };

    const onPageDidFinish = () => {
      // Go to the next page
      setSelectedIndex(selectedIndex + 1);
      // Mark the current page as selected
      setCheckedItems([...checkedItems, selectedIndex]);
    };

    const delegate: LeaseAddFlowDelegate = { onPageDidFinish };

    const leasePartiesComponent = LeaseSelectPartiesContainer(
      LeaseSelectParties,
      delegate,
    )({});
    const leaseFormComponent = createFromExistingLease
      ? LeaseAddExistingFormContainer(LeaseExistingForm, delegate)({})
      : LeaseFurnishedContainer(
        Document,
        delegate,
      )({});
    const leaseConfirmationComponent = LeaseAddConfirmation({
      error: finishFlowWithError,
      title: createFromExistingLease
        ? _("lease_create_existing_success_title")
        : null,
      timerDidEnd: () =>
        router.showPropertySynthesisRoute(selectedProperty?.id, "leases"),
    });

    const componentProps: LeaseAddFlowPageProps = {
      selectedIndex,
      checkedItems,
      allItemsDisabled: finishFlowWithError !== undefined,
      menuItemClick,
      leasePartiesComponent,
      leaseFormComponent,
      leaseConfirmationComponent,
    };

    return WrappedComponent(componentProps);
  };
