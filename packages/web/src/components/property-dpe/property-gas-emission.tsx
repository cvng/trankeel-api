import { Heading, Pane, Text } from "evergreen-ui";
import * as React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";

export type PropertyGasEmissionProps = Record<string, unknown>;

type PropertyGasEmissionItemProps = {
  color: string;
  key: string;
  value: string;
  width: string;
  textColor?: string;
  selected?: boolean;
};

const items: PropertyGasEmissionItemProps[] = [
  {
    color: "#F3EDFC",
    key: "A",
    value: "≤ 5",
    width: "40%",
  },
  {
    color: "#DBC4F4",
    key: "B",
    value: "6 à 10",
    width: "50%",
  },
  {
    color: "#CDAEF1",
    key: "C",
    value: "11 à 20",
    width: "60%",
    selected: true,
  },
  {
    color: "#C399ED",
    key: "D",
    value: "21 à 35",
    width: "70%",
  },
  {
    color: "#AF79E8",
    key: "E",
    value: "36 à 55",
    width: "80%",
  },
  {
    color: "#9B58E3",
    key: "F",
    value: "56 à 80",
    width: "90%",
  },
  {
    color: "#7D30D7",
    textColor: "white",
    key: "G",
    value: "> 80",
    width: "100%",
  },
];

export const PropertyGasEmission: React.FunctionComponent<
  PropertyGasEmissionProps
> = () => {
  const theme = useAppTheme();
  return (
    <Pane maxWidth={200}>
      {items.map((item) => {
        return (
          <Pane
            margin={theme.margin.small}
            padding={theme.margin.small}
            display="flex"
            justifyContent="space-between"
            background={item.color}
            width={item.width}
            style={{
              border: item.selected
                ? `3px solid ${theme.accentColor}`
                : "1px solid black",
            }}
          >
            <Text color={item.textColor}>{item.value}</Text>
            <Heading color={item.textColor} size={500}>{item.key}</Heading>
          </Pane>
        );
      })}
    </Pane>
  );
};
