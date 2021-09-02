import { Card, Heading, Pane, PaneProps } from "evergreen-ui";
import React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import { LottieAnimation } from "../common/lottie-animation";

export type CardSelectorItem = {
  id: string;
  title: string;
  animation?: React.ReactNode;
  selected?: boolean;
  disabled?: boolean;
};

export type CardSelectorProps = {
  items: CardSelectorItem[];
  onClickItem?: (item: CardSelectorItem) => void;
} & PaneProps;

export const CardSelector: React.FunctionComponent<
  CardSelectorProps
> = ({
  items,
  onClickItem,
}) => {
  const theme = useAppTheme();
  const style = (selected: boolean) => {
    return selected
      ? { border: "1px solid " + theme.borderSelectedColor }
      : { border: "1px solid transparent" };
  };
  return (
    <Pane display="flex">
      {items?.map((item, index) =>
        <Card
          key={index}
          elevation={2}
          maxWidth={200}
          textAlign={"center"}
          padding={theme.margin.medium}
          style={{
            cursor: "pointer",
            ...style(item.selected),
          }}
          backgroundColor="white"
          marginRight={theme.margin.large}
          onClick={() => onClickItem?.(item)}
        >
          <LottieAnimation height={100} data={item?.animation} />
          <Heading margin={10}>{item?.title}</Heading>
        </Card>
      )}
    </Pane>
  );
};
