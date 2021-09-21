import { translate } from "piteo-kit";
import React, { createContext, useEffect, useReducer } from "react";
import { AsyncLoadingItemProps } from "../components/loading/async-loading";
import { Closure } from "../utils";

const _ = translate();

const SIMULATION_DURATION = 1200;

export enum AsyncContextAction {
  SetTitle,
  SetActions,
  Hide,
  UpdateAction,
  SimulateAllActionsAsValid,
}

export const AsyncContext = createContext(null);

export interface AsyncContextProps {
  isShown: boolean;
  title: string;
  actions: AsyncLoadingItemProps[];
  simulateAllActionsAsValid: boolean;
  callback?: Closure;
}

export const AsyncProvider = ({ children }) => {
  const initialState = {
    isShown: false,
    title: _("loading"),
    actions: [],
    simulateAllActionsAsValid: false,
  };

  const reducer = (state, action) => {
    switch (action.type) {
      case AsyncContextAction.SetTitle:
        return { ...state, title: action.payload };
      case AsyncContextAction.SetActions:
        return {
          ...state,
          actions: action.payload,
          isShown: action.payload?.length > 0 || false,
        };
      case AsyncContextAction.Hide:
        return {
          ...state,
          isShown: false,
          simulateAllActionsAsValid: false,
          simulateAllActionsAsValidCallback: null,
        };
      case AsyncContextAction.UpdateAction: {
        const actionToUpdate = state.actions?.filter((
          a: AsyncLoadingItemProps,
        ) => a.id === action.payload?.id);
        const idx = state.actions.indexOf(actionToUpdate);
        state.actions[idx] = action.payload;
        return { ...state };
      }
      case AsyncContextAction.SimulateAllActionsAsValid:
        return {
          ...state,
          simulateAllActionsAsValid: true,
          callback: action.payload,
        };
      default:
        return initialState;
    }
  };

  const [state, dispatch] = useReducer(reducer, initialState);

  // Simulate intermediate steps as success logic
  // When the queries are long we display the intermediate steps for the user
  // and once the request is completed we mark all steps as success.
  useEffect(() => {
    const scheduler = setInterval(() => {
      // Get the list of the unchecked actions
      const uncheckedActions = state.actions?.filter((action) =>
        !action.checked
      );
      const shouldHandleUncheckedActions = uncheckedActions.length > 0 &&
        state.actions.length > 0;
      if (shouldHandleUncheckedActions) {
        const idx = state.actions.indexOf(uncheckedActions[0]);
        dispatch({
          type: AsyncContextAction.UpdateAction,
          payload: Object.assign(state.actions[idx], { checked: true }),
        });
      } else if (state.actions.length > 0 && uncheckedActions.length === 0) {
        dispatch({
          type: AsyncContextAction.Hide,
          payload: true,
        });
        clearInterval(scheduler);
        // Call the callback on finish
        state?.callback?.();
      }
    }, SIMULATION_DURATION);
  }, [state, state.simulateAllActionsAsValid, state.actions, dispatch]);

  return (
    <AsyncContext.Provider
      value={{
        isShown: state.isShown,
        title: state.title,
        actions: state.actions,
        dispatch,
      }}
    >
      {children}
    </AsyncContext.Provider>
  );
};
