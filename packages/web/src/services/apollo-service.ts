import { ApolloClient, from, HttpLink, InMemoryCache } from "@apollo/client";
import { setContext } from "@apollo/client/link/context";
import { ErrorResponse, onError } from "@apollo/client/link/error";
// import { getIdToken } from "./auth-service";

const API_URL =
  process.env.NEXT_PUBLIC_API_URL || "https://piteo-api.herokuapp.com/graphql";

interface Context {
  headers: Record<string, string>;
}

class PiteoWebClient extends ApolloClient<unknown> {
  static instance: PiteoWebClient;

  static getInstance(): PiteoWebClient {
    if (!this.instance) {
      this.instance = createApolloClient();
    }
    return this.instance;
  }
}

/// Creates Apollo client.
function createApolloClient(): ApolloClient<unknown> {
  const cache = new InMemoryCache();

  const httpLink = new HttpLink({ uri: API_URL });

  const authLink = setContext(authRequestHandler);

  const errorLink = onError(errorHandler);

  const link = from([authLink, errorLink, httpLink]);

  return new ApolloClient({ cache, link, name: "piteo-web-client" });
}

/// Returns an Apollo context setter for auth token.
async function authRequestHandler(): Promise<Context> {
  // TODO: const authorization = await getIdToken();

  const nextContext: Context = { headers: { /*authorization*/ } };

  return nextContext;
}

/// Returns an Apollo error handler.
function errorHandler(error: ErrorResponse) {
  if (error.graphQLErrors) {
    error.graphQLErrors.forEach(({ message, locations, path }) => {
      console.error(
        `[GraphQL error]: Message: ${message}, Location: ${locations}, Path: ${path}`
      );
    });
  }

  if (error.networkError) {
    console.error(`[Network error]: ${error.networkError}`);
  }
}

/// Returns the Piteo web client.
export function piteoClient(): PiteoWebClient {
  return PiteoWebClient.getInstance();
}
