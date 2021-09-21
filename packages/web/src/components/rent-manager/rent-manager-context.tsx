import { useQuery } from "@apollo/client";
import moment from "moment";
import React, { createContext, useEffect, useReducer } from "react";
import { RentListCountByStatusType } from ".";
import { RentListQuery } from "../../helpers";
import { Rent, RentStatus } from "../../types";
import { DATE_ISO_FORMAT } from "../../validators";

type RentManagerStateProps = {
  loading: boolean;
  originalRentList: Rent[];
  rentList: Rent[];
  selectedRentList: Rent[];
  allRentsSelected: boolean;
  selectedRentStatus: RentStatus;
  groupedRentList: RentListCountByStatusType;
};

export enum RentManagerContextAction {
  Loading,
  RentList,
  AllRentsSelected,
  SelectedRentStatus,
  SelectRent,
  GroupedRentList,
}

export const RentManagerContext = createContext(null);

export const RentManagerProvider = ({ children }) => {
  // Return a rent list filtered by `RentStatus`
  const rentListFilteredByStatus = (
    status: RentStatus,
    rentList: Rent[],
  ): Rent[] => {
    if (!status) {
      return rentList;
    }
    return rentList?.filter((r) => r.status === status);
  };

  // Grouped a rent list by `RentStatus`
  const groupRentListCountByStatus = (
    rentList: Rent[],
  ): RentListCountByStatusType => {
    return {
      all: rentList?.length,
      pending: rentListFilteredByStatus(RentStatus.Pending, rentList)?.length,
      partial: rentListFilteredByStatus(RentStatus.Partial, rentList)?.length,
      settled: rentListFilteredByStatus(
        RentStatus.Settled,
        rentList,
      )?.length,
    };
  };

  const initialState: RentManagerStateProps = {
    loading: true,
    originalRentList: [],
    rentList: [],
    selectedRentList: [],
    allRentsSelected: false,
    selectedRentStatus: null, // We set null by default to show all status
    groupedRentList: { all: 0, pending: 0, partial: 0, settled: 0 },
  };

  const reducer = (
    state: RentManagerStateProps,
    action,
  ): RentManagerStateProps => {
    switch (action.type) {
      case RentManagerContextAction.Loading:
        return { ...state, loading: action.payload };
      case RentManagerContextAction.RentList:
        return {
          ...state,
          originalRentList: action.payload,
          rentList: rentListFilteredByStatus(
            state.selectedRentStatus,
            action.payload,
          ),
          loading: false,
          groupedRentList: groupRentListCountByStatus(action.payload || []),
        };
      case RentManagerContextAction.AllRentsSelected: {
        return {
          ...state,
          allRentsSelected: action.payload,
          selectedRentList: action.payload ? [...state.rentList] : [],
        };
      }
      case RentManagerContextAction.SelectedRentStatus:
        return {
          ...state,
          rentList: rentListFilteredByStatus(
            action.payload,
            state.originalRentList,
          ),
          selectedRentStatus: action.payload,
          selectedRentList: [],
          allRentsSelected: false,
        };
      case RentManagerContextAction.SelectRent: {
        const rent: Rent = action.payload;
        const selected: boolean = action.selected;
        if (selected) {
          return {
            ...state,
            selectedRentList: [...state.selectedRentList, rent],
            allRentsSelected:
              state.selectedRentList?.length === state.rentList.length - 1,
          };
        }
        const selectedRentList = state.selectedRentList;
        const index = selectedRentList?.indexOf(rent);
        if (index > -1) {
          selectedRentList?.splice(index, 1);
        }
        return {
          ...state,
          selectedRentList,
          allRentsSelected: false,
        };
      }
      default:
        return initialState;
    }
  };

  const [state, dispatch] = useReducer(reducer, initialState);

  const { data: { rents } = { rents: [] } } = useQuery(
    RentListQuery,
    {
      variables: {
        until: moment().add(1, "month").startOf("month").format(
          DATE_ISO_FORMAT,
        ),
        since: moment().startOf("month").format(DATE_ISO_FORMAT),
      },
    },
  );

  useEffect(() => {
    if (!rents) return;
    dispatch({ type: RentManagerContextAction.RentList, payload: rents });
  }, [rents]);

  return (
    <RentManagerContext.Provider
      value={{
        dispatch,
        ...state,
      }}
    >
      {children}
    </RentManagerContext.Provider>
  );
};
