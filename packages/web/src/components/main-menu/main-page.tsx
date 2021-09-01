import { Pane } from "evergreen-ui";
import React, { useContext } from "react";
import { MainMenu } from ".";
import { AuthContext } from "../../context/auth-context";

export const MainPage: React.FunctionComponent = ({
  children,
}) => {
  const context = useContext(AuthContext);
  return (
    <Pane display="flex" border="none" height={window.innerHeight}>
      {/* Menu principal */}
      <MainMenu disabled={context.isTrialExpired} />

      {/* Contenu */}
      {children}
    </Pane>
  );
};
