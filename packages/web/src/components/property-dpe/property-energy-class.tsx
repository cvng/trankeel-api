import { Heading, Pane, Text } from "evergreen-ui";
import * as React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";

export type PropertyEnergyClassProps = Record<string, unknown>;

type PropertyEnergyClassItemProps = {
  color: string;
  key: string;
  width: string;
  value?: string;
  textColor?: string;
  selected?: boolean;
};

const items: PropertyEnergyClassItemProps[] = [
  {
    color: "#549A44",
    key: "A",
    width: "40%",
  },
  {
    color: "#67C74D",
    key: "B",
    width: "50%",
  },
  {
    color: "#D7FC5F",
    key: "C",
    width: "60%",
    selected: true,
  },
  {
    color: "#FFFD54",
    key: "D",
    width: "70%",
  },
  {
    color: "#F8CC46",
    key: "E",
    width: "80%",
  },
  {
    color: "#F29E4C",
    key: "F",
    width: "90%",
  },
  {
    color: "#EA3223",
    textColor: "white",
    key: "G",
    width: "100%",
  },
];

export const PropertyEnergyClass: React.FunctionComponent<
  PropertyEnergyClassProps
> = () => {
  const theme = useAppTheme();
  return (
    <Pane maxWidth={200}>
      {items.map((item, index) => {
        return (
          <Pane display="flex" key={index}>
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
                  : "border-width: 50px 0 50px 86.6px",
              }}
            >
              <Text color={item.textColor}>{item.value}</Text>
              <Heading color={item.textColor} size={500}>{item.key}</Heading>
            </Pane>
          </Pane>
        );
      })}
    </Pane>
  );
};
