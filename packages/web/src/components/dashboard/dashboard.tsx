import { Pane } from "evergreen-ui";
import { translate } from "piteo-kit";
import React, { useContext } from "react";
import { Redirect } from "react-router-dom";
import { Routes } from "../../constants";
import { AuthContext } from "../../context/auth-context";
import { eventModel } from "../../helpers/recent-activity-helper";
import { Event } from "../../types";
import { ScrollableContent } from "../common";
import { PageContent } from "../common/page-content";
import { ProfileItem } from "../common/profile-item";
import { Loading } from "../loading";
import { RecentActivityList } from "../recent-activities";
import { RentManager, RentManagerProps } from "../rent-manager/rent-manager";

const _ = translate();

export type DashboardProps = {
  loading: boolean;
  displayOnboarding: boolean;
  rentManagerData: RentManagerProps;
  reventActivityList?: Event[];
};

export const Dashboard: React.FunctionComponent<DashboardProps> = ({
  loading,
  displayOnboarding,
  rentManagerData,
  reventActivityList,
}) => {
  const context = useContext(AuthContext);
  if (displayOnboarding) {
    return <Redirect to={Routes.DASHBOARD_ONBOARDING} />;
  }
  if (loading) {
    return <Loading />;
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
        >
          <RentManager
            {...rentManagerData}
          />

          <RecentActivityList
            loading={loading}
            events={reventActivityList?.map((event: Event) =>
              eventModel(event)
            )}
          />
        </Pane>
      </PageContent>
    </ScrollableContent>
  );
};
