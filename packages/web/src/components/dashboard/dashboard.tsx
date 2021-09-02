import { QueryResult } from "@apollo/client";
import { majorScale, Pane } from "evergreen-ui";
import React, { useContext } from "react";
import { Redirect } from "react-router-dom";
import { Routes } from "../../constants";
import { AuthContext } from "../../context/auth-context";
import { translate } from "piteo-kit";
import {
  Lease,
  Property,
  RentReceivedStatusQuery,
  RentReceivedSummaryQuery,
} from "../../types";
import { ScrollableContent } from "../common";
import { PageContent } from "../common/page-content";
import { ProfileItem } from "../common/profile-item";
import { RentReceivedStatusWidget } from "../widgets/rent-received-status-widget";
import { RentReceivedWidget } from "../widgets/rent-received-widget";

const _ = translate();

export type DashboardProps = {
  rentReceivedSummaryQueryResult: QueryResult<RentReceivedSummaryQuery>;
  rentReceivedStatusQueryResult: QueryResult<RentReceivedStatusQuery>;
  loading: boolean;
  displayOnboarding: boolean;
  editReceipt?: (lease: Lease) => void;
  showRentalContract: (contract: Lease) => void;
  handleMarkRentAsPaid: (paid: boolean, contract: Lease) => void;
};

export const Dashboard: React.FunctionComponent<DashboardProps> = ({
  rentReceivedSummaryQueryResult,
  rentReceivedStatusQueryResult,
  loading,
  displayOnboarding,
  editReceipt,
  showRentalContract,
  handleMarkRentAsPaid,
}) => {
  const context = useContext(AuthContext);
  if (displayOnboarding) {
    return <Redirect to={Routes.DASHBOARD_ONBOARDING} />;
  }
  return (
    <ScrollableContent>
      <PageContent
        title={_("dashboard")}
        titleRightView={<Pane display="flex" flexDirection="row">
          <ProfileItem profile={context?.user} />
        </Pane>}
      >
        <Pane
          display="flex"
          flexDirection="row"
          justifyContent="center"
        >
          <Pane display="flex" flex={1}></Pane>
          <Pane
            display="flex"
            flexDirection="column"
            minWidth={700}
            flex={3}
            justifyContent="center"
          >
            {/* Visite guidée */}
            {
              /* <Tour
              steps={steps}
              accentColor={"#1259B1"}
              isOpen={isTourEnabled}
              rounded={5}
              onRequestClose={() => setTourEnabled(false)}
            /> */
            }

            <Pane>
              <RentReceivedWidget
                loading={rentReceivedSummaryQueryResult?.loading}
                startedDate={new Date(
                  rentReceivedSummaryQueryResult?.data
                    ?.rentReceivedSummary.since,
                )}
                endedDate={new Date(
                  rentReceivedSummaryQueryResult?.data
                    ?.rentReceivedSummary.until,
                )}
                receivedAmount={rentReceivedSummaryQueryResult?.data
                  ?.rentReceivedSummary.amountReceived}
                marginBottom={majorScale(2)}
              />

              {/* Par défaut on affiche au moins un item en mode loading */}
              {rentReceivedStatusQueryResult?.loading
                ? (
                  <RentReceivedStatusWidget loading />
                )
                : (
                  <Pane>
                    {rentReceivedStatusQueryResult?.data?.properties
                      ?.slice()
                      .sort((lhs: Property, rhs: Property) =>
                        rhs.leases?.length - lhs.leases?.length
                      )
                      .map(
                        (property) => {
                          return (
                            <RentReceivedStatusWidget
                              key={property.id}
                              // @ts-ignore: https://github.com/piteo-team/piteo/pull/166
                              property={property}
                              editReceipt={editReceipt}
                              showRentalContract={showRentalContract}
                              markRentAsPaid={handleMarkRentAsPaid}
                              loadingByAction={loading}
                            />
                          );
                        },
                      )}
                  </Pane>
                )}
            </Pane>
          </Pane>
          <Pane display="flex" flex={1}></Pane>
        </Pane>
      </PageContent>
    </ScrollableContent>
  );
};
