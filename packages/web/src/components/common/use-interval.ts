import { useEffect, useRef } from "react";

// https://overreacted.io/making-setinterval-declarative-with-react-hooks

export function useInterval(callback: () => unknown, delay?: number) {
  const savedCallback = useRef();

  // Remember the latest callback.
  useEffect(() => {
    // @ts-ignore: https://github.com/piteo-team/piteo/pull/166
    savedCallback.current = callback;
  }, [callback]);

  // Set up the interval.
  useEffect(() => {
    function tick() {
      // @ts-ignore: https://github.com/piteo-team/piteo/pull/166
      savedCallback.current();
    }
    if (delay !== null) {
      const id = setInterval(tick, delay);
      return () => clearInterval(id);
    }
  }, [delay]);
}
