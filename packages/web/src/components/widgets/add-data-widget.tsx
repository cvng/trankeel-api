import {
  Button,
  Card,
  Heading,
  HomeIcon,
  majorScale,
  Pane,
  PeopleIcon,
  PlusIcon,
  Text,
} from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import { NumberIcon } from "../common/number-icon";

const DISABLED_OPACITY = 0.7;

const _ = translate();

export type AddDataWidgetProps = {
  hasProperty?: boolean;
  isLoadingProperty?: boolean;
  hasTenant?: boolean;
  isLoadingTenant?: boolean;
  hasContract?: boolean;
  isLoadingContract?: boolean;
  onPropertyAddClick?: () => void;
  onTenantAddClick?: () => void;
  onRentalAddClick?: () => void;
};

export const AddDataWidget: React.FunctionComponent<AddDataWidgetProps> = ({
  hasProperty,
  isLoadingProperty,
  hasTenant,
  isLoadingTenant,
  hasContract,
  isLoadingContract,
  onPropertyAddClick,
  onTenantAddClick,
  onRentalAddClick,
}) => {
  const theme = useAppTheme();
  return (
    <Pane>
      {/* Ajout données manuellement */}
      <Card
        display="flex"
        flex={1}
        flexDirection="column"
        elevation={2}
        background="white"
        marginBottom={theme.margin.medium}
        marginTop={theme.margin.large}
      >
        <Pane
          height={majorScale(6)}
          display="flex"
          flexDirection="column"
          justifyContent="center"
          borderBottom="muted"
          paddingX={theme.margin.large}
          marginY={theme.margin.medium}
        >
          <Heading size={500}>{_("add_manual_data")}</Heading>
          <Heading size={200} marginY={theme.margin.medium}>
            {_("add_manual_data_subtitle")}
          </Heading>
        </Pane>
        <Pane display="flex" flex={1}>
          {/* Bien */}
          <Pane
            flex={1}
            display="flex"
            flexDirection="column"
            justifyContent="center"
            alignItems="center"
            borderRight="muted"
            padding={theme.margin.large}
            opacity={1}
          >
            <NumberIcon valid={hasProperty} disabled={false}>
              1
            </NumberIcon>
            <Heading marginTop={majorScale(2)}>
              {_("add_first_property")}
            </Heading>
            <Text marginTop={majorScale(2)} textAlign="center">
              {_("add_first_property_hint")}
            </Text>
            <Heading size={300} marginTop={majorScale(2)} textAlign="center">
              {_("add_first_property_hint_2")}
            </Heading>
            <Button
              appearance={"primary"}
              marginY={majorScale(2)}
              iconBefore={HomeIcon}
              onClick={onPropertyAddClick}
              isLoading={isLoadingProperty}
            >
              {_("add_first_property_btn_title")}
            </Button>
          </Pane>
          {/* Locataire */}
          <Pane
            flex={1}
            display="flex"
            flexDirection="column"
            justifyContent="center"
            alignItems="center"
            borderRight="muted"
            padding={majorScale(2)}
            background={hasProperty ? "white" : "tint1"}
            opacity={hasProperty ? 1 : DISABLED_OPACITY}
          >
            <NumberIcon valid={hasTenant} disabled={!hasProperty}>
              2
            </NumberIcon>
            <Heading marginTop={majorScale(2)}>{_("add_tenant")}</Heading>
            <Text marginTop={majorScale(2)} textAlign="center">
              {_("add_tenant_hint_2")}
            </Text>
            <Heading size={300} marginTop={majorScale(2)} textAlign="center">
              {_("add_tenant_hint_2")}
            </Heading>
            <Button
              appearance="primary"
              marginY={majorScale(2)}
              iconBefore={PeopleIcon}
              disabled={!hasProperty}
              onClick={onTenantAddClick}
              isLoading={isLoadingTenant}
            >
              {_("add_tenant_btn_title")}
            </Button>
          </Pane>
          {/* Location */}
          <Card
            flex={1}
            display="flex"
            flexDirection="column"
            justifyContent="center"
            alignItems="center"
            padding={majorScale(2)}
            background={hasProperty && hasTenant ? "white" : "tint1"}
            opacity={hasProperty && hasTenant ? 1 : DISABLED_OPACITY}
          >
            <NumberIcon
              valid={hasProperty && hasTenant && hasContract}
              disabled={!hasTenant}
            >
              3
            </NumberIcon>
            <Heading marginTop={majorScale(2)}>{_("add_rental")}</Heading>
            <Text marginTop={majorScale(2)} textAlign="center">
              {_("add_rental_hint")}
            </Text>
            <Heading size={300} marginTop={majorScale(2)} textAlign="center">
              {_("add_rental_hint_2")}
            </Heading>
            <Button
              appearance="primary"
              marginY={majorScale(2)}
              iconBefore={PlusIcon}
              disabled={!hasTenant}
              onClick={onRentalAddClick}
              isLoading={isLoadingContract}
            >
              {_("add_rental_btn_title")}
            </Button>
          </Card>
        </Pane>
      </Card>
    </Pane>
  );
};
