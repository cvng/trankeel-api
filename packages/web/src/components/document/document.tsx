import {
  ArrowLeftIcon,
  ArrowRightIcon,
  Badge,
  Button,
  defaultTheme,
  Pane,
  PaneProps,
} from "evergreen-ui";
import { FormikProps } from "formik";
import React from "react";
import { translate } from "piteo-kit";
import { useAppTheme } from "../../hooks/use-app-theme";

const _ = translate();

export type DocumentPageProps = {
  /** Form */
  form: FormikProps<unknown>;
  /** Returns the field for key */
  getItem(key: string): React.ReactNode;
};

export type DocumentProps = {
  pages: React.ReactNode[];
  selectedIndex: number;
  onClickPreviousPage?: () => void;
  onClickNextPage?: () => void;
} & PaneProps;

export const Document: React.FunctionComponent<DocumentProps> = ({
  pages,
  selectedIndex,
  onClickPreviousPage,
  onClickNextPage,
  ...props
}) => {
  const theme = useAppTheme();
  return (
    <Pane flex="1" {...props}>
      {pages.map((item, index) => (
        <Pane
          key={index}
          id={`panel-${index}`}
          role="itempanel"
          aria-hidden={index !== selectedIndex}
          display={index === selectedIndex ? "block" : "none"}
        >
          <Pane
            background="white"
            color={defaultTheme.palette.neutral.base}
            elevation={4}
            padding={theme.margin.largest}
            {...props}
          >
            {item}
          </Pane>
        </Pane>
      ))}
      <Pane
        marginY={theme.margin.large}
        display="flex"
        justifyContent="space-between"
      >
        <Badge color="neutral" isSolid>
          {_("page_number_count", {
            index: selectedIndex + 1,
            count: pages.length,
          })}
        </Badge>
        <Pane>
          <Button
            onClick={onClickPreviousPage}
            height={24}
            iconBefore={ArrowLeftIcon}
            disabled={selectedIndex - 1 === -1 ||
              pages.length === 0 ||
              selectedIndex === 0}
          >
          </Button>
          <Button
            onClick={onClickNextPage}
            height={24}
            iconAfter={ArrowRightIcon}
            disabled={pages.length === 0 ||
              selectedIndex === pages.length - 1}
          />
        </Pane>
      </Pane>
    </Pane>
  );
};
