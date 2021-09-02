import { Card, Pane } from "evergreen-ui";
import React, { useContext } from "react";
import { AuthContext } from "../../context/auth-context";
import { useAppTheme } from "../../hooks/use-app-theme";
import { PageContent } from "../common/page-content";
import { ProfileItem } from "../common/profile-item";

export type SynthesisPageProps = {
  /** Title */
  title: string;
  /** List */
  listComponent: React.ReactNode;
  /** Items */
  items: React.ReactNode[];
};

export const SynthesisPage: React.FunctionComponent<
  SynthesisPageProps
> = ({
  title,
  listComponent,
  items,
}) => {
  const theme = useAppTheme();
  const context = useContext(AuthContext);
  return (
    <PageContent
      title={title}
      titleRightView={<ProfileItem profile={context?.user} />}
    >
      <Card
        padding={theme.margin.medium}
      >
        <Pane display="flex" flex={1} alignItems="flex-start">
          {/* Property list */}
          {listComponent}

          <Pane
            flex={1}
            marginLeft={theme.margin.large}
          >
            {/* Items */}
            {items?.map((item) => {
              return item;
            })}
          </Pane>
        </Pane>
      </Card>
    </PageContent>
  );
};
