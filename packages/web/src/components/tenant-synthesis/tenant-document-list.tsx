import {
  Button,
  Card,
  DownloadIcon,
  Pane,
  PersonIcon,
  Pill,
  Table,
} from "evergreen-ui";
import { fileTypeMap, toLocaleDateString, translate } from "piteo-kit";
import React from "react";
import Skeleton from "react-loading-skeleton";
import animation from "../../assets/lotties/13625-list.json";
import { ColorHelper } from "../../helpers";
import { useAppTheme } from "../../hooks/use-app-theme";
import { File } from "../../types";
import { EmptyDataset } from "../common";
import { LottieAnimation } from "../common/lottie-animation";

const MAX_WIDTH = 240;

const _ = translate();

export type TenantDocumentListProps = {
  loading?: boolean;
  filterEnabled?: boolean;
  files?: File[];
  /** Fired when search field change */
  onSearchFieldChange?: (value: string) => void;
};

export const TenantDocumentList: React.FunctionComponent<
  TenantDocumentListProps
> = ({
  loading,
  filterEnabled,
  files,
  onSearchFieldChange,
}) => {
  const theme = useAppTheme();
  const isEmpty = files?.length === 0;
  const loadingState = () => {
    return <Skeleton count={3} height={40} />;
  };
  const noDataState = () => {
    return <EmptyDataset
      title={_("no_document_title")}
      subtitle={_("no_document_subtitle")}
      animation={<LottieAnimation data={animation} />}
    />;
  };
  const emptyState = () => {
    return <Pane display="flex" justifyContent="center">
      <EmptyDataset
        width={MAX_WIDTH}
        title={_("no_document_title")}
        subtitle={_("no_document_no_result_subtitle")}
        icon={PersonIcon}
        removeBorder={false}
      />
    </Pane>;
  };
  const shouldDisplayNoData = !loading && isEmpty && !filterEnabled;
  const shouldDisplayNoResult = !loading && isEmpty && filterEnabled;
  return (
    <Card
      background="white"
      padding={theme.margin.large}
      elevation={2}
    >
      {/* Loading */}
      {loading
        ? (
          loadingState()
        )
        : (
          // No data
          shouldDisplayNoData
            ? (
              noDataState()
            )
            : (
              <Table>
                <Table.Head>
                  <Table.SearchHeaderCell
                    placeholder={_("search_document_placeholder")}
                    onChange={(e) => onSearchFieldChange?.(e)}
                  />
                  <Table.TextHeaderCell>{_("type")}</Table.TextHeaderCell>
                  <Table.TextHeaderCell>
                    {_("creation_date")}
                  </Table.TextHeaderCell>
                  <Table.TextHeaderCell></Table.TextHeaderCell>
                </Table.Head>
                <Table minHeight={500}>
                  {shouldDisplayNoResult
                    ? (
                      emptyState()
                    )
                    : (
                      files?.map((item, index) => (
                        <Table.Row
                          key={index}
                          isSelectable
                        >
                          <Table.TextCell>
                            {_("rent_receipt_filename", item)}
                          </Table.TextCell>
                          <Table.TextCell>
                            <Pill color={ColorHelper.fileMapColor(item.type)}>
                              {fileTypeMap().get(item.type) || "-"}
                            </Pill>
                          </Table.TextCell>
                          <Table.TextCell>
                            {toLocaleDateString(item?.createdAt, "-")}
                          </Table.TextCell>
                          <Table.TextCell>
                            <Button
                              iconBefore={DownloadIcon}
                              onClick={() =>
                                window.open(item?.downloadUrl, "_blank")}
                            >
                              {_("download")}
                            </Button>
                          </Table.TextCell>
                        </Table.Row>
                      ))
                    )}
                </Table>
              </Table>
            )
        )}
    </Card>
  );
};
