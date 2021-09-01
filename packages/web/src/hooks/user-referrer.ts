import { useLocation } from "react-router-dom";

const REFERRER_KEY = "referrer";

export const useReferrer = () => {
  return new URLSearchParams(useLocation().search).get(REFERRER_KEY);
};
