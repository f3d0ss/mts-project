export const getDateFromTimestamp = (timestamp: number) => {
  const milliseconds = timestamp * 1000;
  return new Date(milliseconds);
};
