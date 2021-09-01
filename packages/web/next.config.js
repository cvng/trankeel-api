module.exports = {
  reactStrictMode: true,
  rewrites: async () => {
    return [
      {
        source: "/:any*",
        destination: "/",
      },
    ];
  },
};
