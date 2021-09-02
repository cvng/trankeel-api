import { useQuery } from "@apollo/client";
import * as firebase from "firebase";
import { translate } from "piteo-kit";
import React, { createContext, useEffect, useReducer } from "react";
import { Loading } from "../components/loading/loading";
import { UserQuery } from "../helpers";
import { User } from "../types";

const _ = translate("common");

enum AuthContextAction {
  Loading = "loading",
  ApiUser = "api-user",
  ApiUserError = "api-user-error",
  FirebaseUser = "firebase-user",
}

export const AuthContext = createContext(null);

export interface AuthContextProps {
  isTrialExpired: boolean;
  user: User;
}

export const AuthProvider = ({ children }) => {
  const initialState = {
    loading: true,
    user: { api: null, firebase: null },
  };

  const reducer = (state, action) => {
    switch (action.type) {
      case AuthContextAction.Loading:
        return { ...state, loading: action.payload };
      case AuthContextAction.FirebaseUser:
        return {
          ...state,
          user: { firebase: action.payload, api: null },
          loading: !!action.payload,
        };
      case AuthContextAction.ApiUser:
        return {
          ...state,
          user: {
            api: action.payload,
          },
          loading: false,
        };
      case AuthContextAction.ApiUserError:
        return {
          ...initialState,
          loading: false,
        };
      default:
        return initialState;
    }
  };

  const [state, dispatch] = useReducer(reducer, initialState);

  // Prepare the user query without sending it
  // we wait for the user to be connected in order to send the request
  const { error: apiUserError, data: { user: apiUser } = { user: null } } =
    useQuery(
      UserQuery,
      {
        skip: !state.user.firebase, // Launch the query only when the firebase user is set
      },
    );

  // Step 1 - Perform only once the listener on auth state changed
  useEffect(() => {
    firebase.auth().onAuthStateChanged((firebaseUser: firebase.User) => {
      dispatch({
        type: AuthContextAction.FirebaseUser,
        payload: firebaseUser,
      });
    });
  }, []);

  // Step 2 - Check the trial period
  useEffect(() => {
    if (!apiUser && !apiUserError) return;
    if (apiUserError) {
      dispatch({ type: AuthContextAction.ApiUserError });
      return;
    }
    // Assign the user api : now we are authenticated
    dispatch({ type: AuthContextAction.ApiUser, payload: apiUser });
  }, [apiUser, apiUserError]);

  if (state.loading) {
    return <Loading title={_("loading")} />;
  }

  return (
    <AuthContext.Provider
      value={{
        isTrialExpired: false,
        user: state.user.api,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};
