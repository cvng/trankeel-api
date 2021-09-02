import React, { createContext, useReducer } from "react";
import { Property, Tenant } from "../../types";

export enum LeaseAddContextAction {
  CreateFromExistingLease = "create-from-existing-lease",
  SetProperty = "set-property",
  SetTenants = "set-tenants",
  SetFlowFinishWithError = "set-flow-finish-with-error",
  Reset = "reset",
}

export const LeaseAddContext = createContext(null);

export interface LeaseAddContextProps {
  dispatch: (data: { type: LeaseAddContextAction; payload: unknown }) => void;
  createFromExistingLease?: boolean;
  selectedProperty?: Property;
  selectedTenants?: Tenant[];
  finishFlowWithError?: boolean;
}

export const LeaseAddProvider = ({ children }) => {
  const initialState = {
    createFromExistingLease: true,
    selectedProperty: null,
    selectedTenants: [],
    finishFlowWithError: undefined,
  };

  const reducer = (state, action) => {
    switch (action.type) {
      case LeaseAddContextAction.CreateFromExistingLease:
        return { ...initialState, createFromExisting: action.payload };
      case LeaseAddContextAction.SetProperty:
        return { ...state, selectedProperty: action.payload };
      case LeaseAddContextAction.SetTenants:
        return { ...state, selectedTenants: action.payload };
      case LeaseAddContextAction.SetFlowFinishWithError:
        return { ...state, finishFlowWithError: action.payload };
      case LeaseAddContextAction.Reset:
        return { ...initialState };
      default:
        return initialState;
    }
  };

  const [state, dispatch] = useReducer(reducer, initialState);

  return (
    <LeaseAddContext.Provider
      value={{ dispatch, ...state }}
    >
      {children}
    </LeaseAddContext.Provider>
  );
};
