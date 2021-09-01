import { ApolloProvider } from "@apollo/client";
import { Elements } from "@stripe/react-stripe-js";
import { AppProps } from "next/app";
import { piteoClient } from "../services/apollo-service";
import { loadFirebase } from "../services/firebase-service";
import { stripePromise } from "../services/stripe-service";
import "../styles/globals.css";

if (typeof window !== "undefined") {
  loadFirebase();
}

const MyApp = ({ Component, pageProps }: AppProps) => {
  return (
    <ApolloProvider client={piteoClient()}>
      <Elements stripe={stripePromise()}>
          <Component {...pageProps} />
        </Elements>
    </ApolloProvider>
  );
};

export default MyApp;
