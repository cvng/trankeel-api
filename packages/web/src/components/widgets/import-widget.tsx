import {
  Button,
  Card,
  Heading,
  HomeIcon,
  majorScale,
  Pane,
  Text,
} from "evergreen-ui";
import React from "react";
import Lottie from "react-lottie";
import animationData from "../../assets/lotties/woman-flowers.json";
import { translate } from "piteo-kit";

const _ = translate();

const defaultOptions = {
  loop: true,
  autoplay: true,
  animationData: animationData,
  rendererSettings: {
    preserveAspectRatio: "xMidYMid meet",
  },
};

export type ImportWidgetProps = {
  onImportFromExistingSolutionClick?: () => void;
};

export const ImportWidget: React.FunctionComponent<ImportWidgetProps> = ({
  onImportFromExistingSolutionClick,
  ...props
}) => {
  return (
    <Pane {...props}>
      {/* Import */}
      <Card
        display="flex"
        flexDirection="column"
        elevation={2}
        marginTop={majorScale(3)}
        background="white"
      >
        <Pane
          height={majorScale(6)}
          display="flex"
          flexDirection="column"
          justifyContent="center"
          borderBottom="muted"
          paddingX={majorScale(2)}
        >
          <Heading size={500}>{_("import_existing_patrimony")}</Heading>
        </Pane>
        <Pane display="flex" flex={1}>
          <Pane
            flex={1}
            display="flex"
            flexDirection="column"
            justifyContent="center"
            alignItems="center"
            borderRight="muted"
            padding={majorScale(2)}
          >
            <Heading>{_("import_data")}</Heading>
            <Text marginTop={majorScale(2)} textAlign="center">
              {_("import_intent_help_1")}
            </Text>
            <Heading size={300} marginTop={majorScale(2)} textAlign="center">
              {_("import_intent_help_2")}
            </Heading>
            <Pane
              display="flex"
              flexDirection="row"
              justifyContent="space-around"
              alignItems="center"
              marginTop={majorScale(2)}
            >
              <Button
                appearance="primary"
                intent="success"
                iconBefore={HomeIcon}
                onClick={onImportFromExistingSolutionClick}
                marginRight={majorScale(2)}
              >
                {_("import_patrimony")}
              </Button>
            </Pane>
          </Pane>
          <Pane margin={majorScale(2)}>
            <Lottie options={defaultOptions} height={220} width={220} />
          </Pane>
        </Pane>
      </Card>
    </Pane>
  );
};
