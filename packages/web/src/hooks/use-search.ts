// Filter the list on the property returned by the fn parameter
export const useSearch = <T>(
  fn: (item: T) => string,
  list?: T[],
  predicate?: string,
) => {
  if (!predicate || list?.length === 0) {
    return list;
  }
  return list?.filter((item) => {
    return fn(item)?.toLowerCase().includes(
      predicate?.toString()?.toLowerCase(),
    );
  });
};
