(() => {
  async function start(): Promise<void> {
    const pkg = await import('../wasm/pkg');

    pkg.start(process.env.GRPC_HOST!);
  }

  // eslint-disable-next-line no-console
  start().catch(console.error);
})();
