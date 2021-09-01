export const DATE_FORMAT = "DD/MM/YYYY";
export const DATE_MONTH_FORMAT = "MMMM YYYY";

// Just a simple closure without params
export type Closure = () => void;

// A closure with an id as unique param
export type UniquableClosure = (id: string) => void;

// Unwrapped objects
export function uncast(value: unknown) {
  // Convert falsy values to empty strings for inputs usage.
  const reviver = (key: string, val: unknown) => {
    if (typeof val === "undefined") return "";
    if (val && key === "amount" && typeof val !== "object") return val;
    return val;
  };
  try {
    return JSON.parse(JSON.stringify(value), reviver);
  } catch {
    return value;
  }
}

// Error handling
export function code(error: unknown): string {
  // Extract code from an error object.
  return (
    // deno-lint-ignore no-explicit-any
    (error as any)?.code ??
    // deno-lint-ignore no-explicit-any
    (error as any)?.graphQLErrors?.[0]?.extensions.code?.toLowerCase() ??
    "error_smi"
  );
}

// Environment
export function isDemoEnv(): boolean {
  return Boolean(
    window.location.href.includes("demo.piteo.fr") ||
      window.location.href.includes("?demo") ||
      process.env.NEXT_PUBLIC_FORCE_DEMO
  );
}

// Routing
export const isSelectedRoute = (url: string, link: string): boolean => {
  return url === link || url.includes(link);
};
