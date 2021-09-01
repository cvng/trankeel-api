import { ApolloProvider } from "@apollo/client";
import { AppProps } from "next/app";
import { loadFirebase } from "../services/firebase-service";
import { piteoClient } from "../services/apollo-service";
import "../styles/globals.css";

if (typeof window !== "undefined") {
  loadFirebase();
}

const MyApp = ({ Component, pageProps }: AppProps) => {
  return (
    <ApolloProvider client={piteoClient()}>
      <Component {...pageProps} />
    </ApolloProvider>
  );
};

export default MyApp;
