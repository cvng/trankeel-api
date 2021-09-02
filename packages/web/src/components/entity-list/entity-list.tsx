import {
  AddIcon,
  Button,
  majorScale,
  Pane,
  PaneProps,
  PlusIcon,
  SearchIcon,
  SearchInput,
  Text,
} from "evergreen-ui";
import React from "react";
import Skeleton from "react-loading-skeleton";
import { translate } from "piteo-kit";
import { useAppTheme } from "../../hooks/use-app-theme";
import { CardItem } from "../cards/card-item";
import { EmptyDataset } from "../common";
import { LottieAnimation } from "../common/lottie-animation";

const _ = translate();

const MAX_WIDTH = 240;
const BUTTON_HEIGHT = 38;

export type EntityListProps<T> = {
  entities?: T[];
  loading?: boolean;
  filterEnabled?: boolean;
  entity: string; // Name of the entity
  emptyDatasetIcon?: React.ElementType;
  emptyDatasetAnimation?: React.ReactNode;
  cardData: (entity: T) => {
    key: string;
    title: string;
    subtitle?: string;
    selected: boolean;
  };
  onSelectEntity?: (entityId: string) => void;
  onSearchFieldChange?: (value: string) => void;
  onAddNewEntity?: () => void;
} & PaneProps;

export const EntityList: React.FunctionComponent<
  EntityListProps<{ id: string }>
> = ({
  entities,
  loading,
  filterEnabled,
  entity,
  emptyDatasetIcon,
  emptyDatasetAnimation,
  cardData,
  onSelectEntity,
  onSearchFieldChange,
  onAddNewEntity,
  ...props
}) => {
  const theme = useAppTheme();
  const shouldDisplayEmptyDataset = entities?.length === 0 && !filterEnabled;
  const shouldDisplayNoResult = entities?.length === 0 && filterEnabled;

  if (loading) {
    return <Pane width={MAX_WIDTH}>
      <Skeleton count={3} height={40} />
    </Pane>;
  }
  entity = entity?.toLocaleLowerCase();

  if (shouldDisplayEmptyDataset) {
    return <EmptyDataset
      width={MAX_WIDTH}
      title={_("no_entity_available_title", { entity })}
      subtitle={_("no_entity_available_subtitle", { entity })}
      animation={!!emptyDatasetAnimation &&
        <LottieAnimation height={150} data={emptyDatasetAnimation} />}
      icon={emptyDatasetIcon}
      removeBorder={false}
      button={<Button
        iconBefore={PlusIcon}
        onClick={onAddNewEntity}
        appearance="primary"
      >
        {_("action_add_entity", { entity })}
      </Button>}
    />;
  }

  const noResultItem = <EmptyDataset
    title={_("no_entity_filter_available_title")}
    subtitle={_("no_entity_filter_available_subtitle", { entity })}
    icon={SearchIcon}
  />;

  return (
    <Pane
      display="flex"
      flexDirection="column"
      justifyContent="flex-start"
      flex={1}
      {...props}
      maxWidth={MAX_WIDTH}
    >
      <Button
        iconBefore={AddIcon}
        onClick={onAddNewEntity}
        appearance="primary"
        height={BUTTON_HEIGHT}
        width={MAX_WIDTH}
      >
        {_("action_add_entity", { entity })}
      </Button>

      <SearchInput
        height={BUTTON_HEIGHT}
        placeholder="Recherche"
        marginY={majorScale(2)}
        width={MAX_WIDTH}
        onChange={(e) => onSearchFieldChange?.(e.target.value)}
        zIndex={0}
      />
      {/* List items */}
      {entities?.map((entityItem) => {
        return (<CardItem
          {...cardData(entityItem)}
          marginBottom={theme.margin.medium}
          onClick={() => onSelectEntity?.(entityItem.id)}
        />);
      })}
      {/* Result count */}
      {entities?.length > 0 &&
        <Pane display="flex" alignItems="flex-end" justifyContent="center">
          <Text display="flex" alignItems="flex-end" size={300}>
            {entities?.length === 1
              ? _("result_count", { count: entities?.length, entity })
              : _("results_count", { count: entities?.length, entity })}
          </Text>
        </Pane>}
      {shouldDisplayNoResult && noResultItem}
    </Pane>
  );
};
