import { PHRASES } from "./locales/fr/phrases";

type Vars = Record<string, unknown>;

export function translate(namespace = "", phrases = PHRASES) {
  return function (key: string, vars?: Vars) {
    // @ts-ignore: No index signature.
    if (phrases[namespace] && phrases[namespace][key]) {
      // @ts-ignore: No index signature.
      return interpolate(phrases[namespace][key], vars);
      // @ts-ignore: No index signature.
    } else if (phrases[key]) {
      // @ts-ignore: No index signature.
      return interpolate(phrases[key], vars);
    } else {
      throw new Error(`Missing translation key: ${namespace}.${key}`);
    }
  };
}

export function capitalize(value: string | undefined) {
  if (!value) return "";
  return value.charAt(0).toUpperCase() + value.slice(1);
}

function interpolate(str: string, vars?: Vars) {
  if (vars) {
    for (const [key, value] of Object.entries(vars)) {
      str = str.replace(`{{${key}}}`, String(value));
    }
  }
  return str;
}
