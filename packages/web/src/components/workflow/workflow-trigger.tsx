import {
  AddIcon,
  BuildIcon,
  Card,
  Combobox,
  Heading,
  Icon,
  IconButton,
  minorScale,
  MoreIcon,
  Pane,
} from "evergreen-ui";
import React from "react";

export type WorkflowTriggerProps = {
  hasAction?: boolean;
};

const cardStyle = { border: "1px solid #EBEDF3" };

const style = {
  display: "flex",
  alignItems: "center",
  padding: minorScale(3),
  flex: 1,
};

export const WorkflowTrigger: React.FunctionComponent<
  WorkflowTriggerProps
> = ({
  hasAction,
}) => {
  return (
    <Pane
      minHeight={100}
      display="flex"
      flexDirection="column"
      alignItems="center"
    >
      <Card
        display="flex"
        flexDirection="column"
        minWidth={300}
        style={{
          ...cardStyle,
        }}
      >
        <Pane
          style={{
            ...style,
            justifyContent: "space-between",
          }}
        >
          <Pane display="flex">
            <Icon icon={BuildIcon} color="info" marginRight={minorScale(3)} />
            <Heading size={300}>DECLENCHEUR</Heading>
          </Pane>
          <Pane>
            <IconButton height={24} icon={MoreIcon} />
          </Pane>
        </Pane>
        <Pane
          style={{
            ...style,
            borderTop: "1px solid #EBEDF3",
          }}
        >
          <Combobox
            openOnFocus
            width="100%"
            items={[
              "Revenus - Loyer marqué comme payé",
              "Contrat - Nouvelle location",
              "Contrat - Fin d'un contrat",
            ]}
            onChange={(selected) => console.log(selected)}
            placeholder="Sélectionner un déclencheur"
          />
        </Pane>
      </Card>
      <Pane
        display="flex"
        flexDirection="column"
        alignItems="center"
        width={30}
      >
        <Pane background={"#EBEDF3"} width={2} minHeight={20}></Pane>
        <Icon icon={AddIcon} color="info" size={20} />
        {hasAction &&
          <Pane background={"#EBEDF3"} width={2} minHeight={20}></Pane>}
      </Pane>
    </Pane>
  );
};
