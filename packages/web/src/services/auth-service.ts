import * as firebase from "firebase";
import LogRocket from "logrocket";
import { translate } from "piteo-kit";
import { AccountCreateMutation, UserQuery } from "../helpers";
import { UserWithAccountInput } from "../types";
import { piteoClient } from "./apollo-service";

const _ = translate();

// https://github.com/firebase/firebase-js-sdk/blob/master/packages-exp/auth-exp/src/core/errors.ts
const ERROR_CODE_ALREADY_EXISTS = "auth/email-already-in-use";
const ERROR_CODE_USER_NOT_FOUND = "auth/user-not-found";
const ERROR_CODE_WRONG_PASSWORD = "auth/wrong-password";
const ERROR_CODE_NETWORK_FAILED = "auth/network-request-failed";

type Token = string;

export enum AuthErrorCode {
  Smi,
  ExistsAccount,
  CancelledByUser,
  WrongCredentials,
  NetworkFailed,
}

export class AuthError extends Error {
  constructor(public code: AuthErrorCode, public message: string) {
    super();
  }
}

/// Returns a JSON Web Token (JWT) used to identify the user.
export async function getIdToken(): Promise<Token> {
  try {
    return await firebase.auth().currentUser?.getIdToken(true);
  } catch (error) {
    throw formatError(error);
  }
}

/// Asynchronously signs in using an email and password.
export async function signInWithEmailAndPassword(
  email: string,
  password: string,
  rememberMe?: boolean
): Promise<firebase.auth.UserCredential> {
  try {
    const client = piteoClient();

    await firebase
      .auth()
      .setPersistence(
        rememberMe
          ? firebase.auth.Auth.Persistence.LOCAL
          : firebase.auth.Auth.Persistence.SESSION
      );

    const userCredential = await firebase
      .auth()
      .signInWithEmailAndPassword(email.toLowerCase(), password);

    const userQueryResponse = await client.query({
      query: UserQuery,
    });
    if (userQueryResponse?.error?.message != null) {
      throw new Error(userQueryResponse.error.message);
    }

    await setAuthenticated(userCredential.user);

    return userCredential;
  } catch (error) {
    throw formatError(error);
  }
}

/// Creates a new user account associated with the specified email address and password.
export async function createUserWithAccount(
  firstName: string,
  lastName: string,
  email: string,
  password: string
): Promise<firebase.auth.UserCredential> {
  try {
    const userCredential = await firebase
      .auth()
      .createUserWithEmailAndPassword(email.toLowerCase(), password);

    // Update the display name in firebase.
    const displayName = [firstName, lastName].join(" ").trim();
    await firebase.auth().currentUser.updateProfile({ displayName });

    // Create the account in the backend.
    const accountInput: UserWithAccountInput = {
      firstName,
      lastName,
      email,
      authId: userCredential.user.uid,
    };
    await piteoClient().mutate({
      mutation: AccountCreateMutation,
      variables: { input: accountInput },
    });

    await setAuthenticated(userCredential.user);

    return userCredential;
  } catch (error) {
    throw formatError(error);
  }
}

/// Sends a password reset email to the given email address.
export async function resetPassword(email: string): Promise<void> {
  try {
    return await firebase.auth().sendPasswordResetEmail(email.toLowerCase());
  } catch (error) {
    throw formatError(error);
  }
}

/// Signs out the current user and reset cache.
export async function signOut(): Promise<void> {
  try {
    await firebase.auth().signOut();
    await piteoClient().resetStore();
  } catch (error) {
    throw formatError(error);
  }
}

/// Sets the current user as authenticated.
async function setAuthenticated(user: firebase.User): Promise<Token> {
  try {
    // Start by renewing the token if necesary.
    const idToken = await user.getIdToken(true);

    if (process.env.NODE_ENV === "production") {
      // Identify the user for Crisp session.
      // $crisp.push(["set", "user:email", [user.email]]);
      // $crisp.push(["set", "user:nickname", [user.displayName]]);

      // Identify the user for Logrocket session.
      LogRocket.identify(user.uid, {
        email: user.email,
        name: user.displayName,
      });
    }

    return idToken;
  } catch (error) {
    throw formatError(error);
  }
}

/// Transforms error.
function formatError(error: firebase.auth.Error): AuthError {
  switch (error.code) {
    case ERROR_CODE_ALREADY_EXISTS:
      return new AuthError(
        AuthErrorCode.ExistsAccount,
        _("error_existing_account")
      );

    case ERROR_CODE_USER_NOT_FOUND:
      return new AuthError(
        AuthErrorCode.WrongCredentials,
        _("error_invalid_username")
      );

    case ERROR_CODE_WRONG_PASSWORD:
      return new AuthError(
        AuthErrorCode.WrongCredentials,
        _("error_invalid_password")
      );

    case ERROR_CODE_NETWORK_FAILED:
      return new AuthError(AuthErrorCode.NetworkFailed, _("error_net"));

    default:
      return new AuthError(AuthErrorCode.Smi, _("error_smi"));
  }
}
