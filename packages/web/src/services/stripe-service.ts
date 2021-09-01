import { loadStripe } from "@stripe/stripe-js";

export function stripePromise() {
  return loadStripe(process.env.NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY as string);
}
