import * as webpack from 'webpack';
import * as path from 'path';
import * as RawWasmPackPlugin from '@wasm-tool/wasm-pack-plugin';
import { CleanWebpackPlugin } from 'clean-webpack-plugin';
import TsconfigPathsPlugin from 'tsconfig-paths-webpack-plugin';
import * as HtmlEntryLoader from 'html-entry-loader';
import * as GrpcWebPlugin from 'grpc-webpack-plugin';

const WasmPackPlugin = RawWasmPackPlugin as unknown as new (options: RawWasmPackPlugin.WasmPackPluginOptions) => webpack.WebpackPluginInstance;

const root = path.resolve(__dirname, '..');
const dist = path.resolve(root, 'client/dist');

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

    new GrpcWebPlugin({
      protoPath: path.resolve(root, 'proto'),
      protoFiles: ['auth.proto'],
      outputType: 'grpc-web',
      importStyle: 'typescript',
      binary: true,
      outDir: path.resolve('client/src/proto'),
      extra: [`--js_out=import_style=commonjs,binary:${path.resolve('client/src/proto')}`],
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(root, 'client/wasm'),
      outDir: path.resolve(root, 'client/wasm/pkg'),
      outName: 'index',
    }),
    new CleanWebpackPlugin(),
  ],
};

export default configuration;
