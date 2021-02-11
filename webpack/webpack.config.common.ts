import * as webpack from 'webpack';
import * as path from 'path';
import { CleanWebpackPlugin } from 'clean-webpack-plugin';
import TsconfigPathsPlugin from 'tsconfig-paths-webpack-plugin';
import * as HtmlEntryLoader from 'html-entry-loader';
import * as WebpackShellPluginNext from 'webpack-shell-plugin-next';

// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-var-requires
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const root = path.resolve(__dirname, '..');
const dist = path.resolve(root, 'client/dist');

const grpcWebBuild = `node node_modules/protoc-gen-grpc/bin/protoc-gen-grpc.js -I=${path.resolve(root, 'node_modules/protoc/protoc/include')} -I=${path.resolve(root, 'proto')} auth.proto`
  + ` --js_out=import_style=commonjs,binary:${path.resolve('client/src/proto')}`
  + ` --grpc-web_out=import_style=typescript,mode=grpcweb:${path.resolve('client/src/proto')}`;

const configuration: webpack.Configuration = {
  context: root,
  entry: {
    client: 'client/src/client.html',
  },
  experiments: {
    asyncWebAssembly: true,
  },
  output: {
    path: dist,
    filename: '[name].js',
  },
  module: {
    rules: [
      {
        test: /\.(html)$/,
        use: [
          {
            loader: 'html-entry-loader',
            options: {
              minimize: true,
            },
          },
        ],
      },
      {
        test: /\.ts$/,
        use: [
          {
            loader: 'ts-loader',
            options: {
              onlyCompileBundledFiles: true,
              compilerOptions: {
                module: 'esnext',
              },
            },
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: ['.ts', '.js'],
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    plugins: [new TsconfigPathsPlugin() as any],
    alias: {
      proto: path.resolve(root, 'client/src/proto'),
    },
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new HtmlEntryLoader.EntryExtractPlugin(),

    // eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/no-unsafe-call
    new (WebpackShellPluginNext as any)({
      onBeforeNormalRun: {
        scripts: [grpcWebBuild],
      },
      onWatchRun: {
        scripts: [grpcWebBuild],
      },
      onBeforeBuild: {
        scripts: [grpcWebBuild],
      },
    }),

    // eslint-disable-next-line @typescript-eslint/no-unsafe-call
    new WasmPackPlugin({
      crateDirectory: path.resolve(root, 'client/wasm'),
      outDir: path.resolve(root, 'client/wasm/pkg'),
      outName: 'index',
    }),
    new CleanWebpackPlugin(),
  ],
};

export default configuration;
