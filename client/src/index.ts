(() => {
  function getGrpcHost(): string {
    if (process.env.IS_LOCALHOST) return 'http://localhost:8000';
    return 'https://account-server.dlunch.net';
  }

  async function start(): Promise<void> {
    const pkg = await import('../wasm/pkg');

    pkg.start(getGrpcHost());
  }

  // eslint-disable-next-line no-console
  start().catch(console.error);
})();
