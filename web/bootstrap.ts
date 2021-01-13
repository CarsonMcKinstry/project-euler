(async () => {
  try {
    const module = await import("./main");
    const { main } = module;
    await main();
  } catch (err) {
    console.log(`An error occurred while attempting to run main: ${err}`);
  }
})();
