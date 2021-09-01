import { Pane } from "evergreen-ui";
import React, { useContext } from "react";
import { AuthContext } from "../../context/auth-context";
import { PageContent } from "../common";
import { ProfileItem } from "../common/profile-item";
import { BillingSubscribe, BillingSubscribeProps } from "./billing-subscribe";
import { translate } from "piteo-kit";

const _ = translate();

export type BillingSubscribePageProps = BillingSubscribeProps;

export const BillingSubscribePage: React.FunctionComponent<
  BillingSubscribePageProps
> = ({
  processingPayment,
  plan,
  loading,
  form,
  validationSchema,
}) => {
  const context = useContext(AuthContext);
  return (
    <PageContent
      title={_("subscription")}
      titleRightView={<Pane display="flex" flexDirection="row">
        <ProfileItem profile={context?.user} />
      </Pane>}
    >
      <BillingSubscribe
        plan={plan}
        loading={loading}
        processingPayment={processingPayment}
        form={form}
        validationSchema={validationSchema}
      />
    </PageContent>
  );
};
