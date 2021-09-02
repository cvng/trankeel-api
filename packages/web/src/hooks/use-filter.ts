// Filter the list by removing the entries that don't match de predicate condition
export const useFilter = <T>(
  list: T[],
  predicate: (item: T) => boolean,
) => {
  if (list?.length === 0) {
    return list;
  }
  return list?.filter((item) => predicate(item));
};
