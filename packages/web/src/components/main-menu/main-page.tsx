import { Pane } from "evergreen-ui";
import React, { useContext } from "react";
import { MainMenu } from ".";
import { AsyncContext } from "../../context/async-context";
import { AuthContext } from "../../context/auth-context";
import { AsyncLoading } from "../loading";

export const MainPage: React.FunctionComponent = ({
  children,
}) => {
  const context = useContext(AuthContext);
  const payload = useContext(AsyncContext);
  return (
    <Pane display="flex" border="none" height={window.innerHeight}>
      {/* Menu principal */}
      <MainMenu disabled={context.isTrialExpired} />

      {/* Contenu */}
      {children}

      {/* Modal asynchrone globale */}
      <AsyncLoading {...payload} />
    </Pane>
  );
};
