import { ApolloProvider } from "@apollo/client";
import { AppProps } from "next/app";
import { piteoClient } from "../services/apollo-service";
import "../styles/globals.css";

const MyApp = ({ Component, pageProps }: AppProps) => {
  return (
    <ApolloProvider client={piteoClient()}>
      <Component {...pageProps} />
    </ApolloProvider>
  );
};

export default MyApp;
