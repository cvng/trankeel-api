import React, { useContext } from "react";
import { Redirect, Route, useLocation } from "react-router-dom";
import { AuthContext } from "../context/auth-context";
import { Routes } from "../constants";

export const AuthenticatedRoute = ({ component: Component, ...rest }) => {
  const { user, isTrialExpired } = useContext(AuthContext);
  // If the user tried to access an authenticated route while no authenticated
  // we set the referrer to the url and once authenticated the user will be
  // redirect to the referrer url
  let redirectURL: string = Routes.DEFAULT;
  const referrer = useLocation().pathname;
  if (referrer?.length > 0) {
    redirectURL = Routes.LOGIN + "?referrer=" + referrer;
  }
  const renderComponent = (props) => {
    if (user) {
      // Check if is trial expired
      // prevent to block redirecting on subscribe
      const whitelistRoutes = [Routes.SUBSCRIBE];
      const followingRoute = props?.location?.pathname;
      const isAllowed = whitelistRoutes.includes(followingRoute);
      if (isTrialExpired && !isAllowed) {
        // Go to the suscribe screen
        return <Redirect to={Routes.SUBSCRIBE} />;
      }
      // Get access to the component
      return <Component {...props} />;
    } else {
      return <Redirect to={redirectURL} />;
    }
  };
  return (
    <Route
      {...rest}
      render={renderComponent}
    />
  );
};
