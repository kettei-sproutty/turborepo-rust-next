import { join } from "node:path";
import { access, symlink } from "node:fs/promises";

/**
 * @type {import('next').NextConfig}
 */
const nextConfig = {
  reactStrictMode: true,
  experimental: {
    appDir: true,
  },
  transpilePackages: ["greet", "ui"],
  webpack(config, { isServer, dev }) {
    // Use the client static directory in the server bundle and prod mode
    // Fixes `Error occurred prerendering page "/"`
    config.output.webassemblyModuleFilename =
        isServer && !dev ? "../static/wasm/[modulehash].wasm" : "static/wasm/[modulehash].wasm";

    // Since Webpack 5 doesn't enable WebAssembly by default, we should do it manually
    config.experiments = { ...config.experiments, asyncWebAssembly: true };

    /**
     * Create a symlink from the client static directory to the server static directory
     * @see https://github.com/vercel/next.js/issues/25852#issuecomment-1057059000
     * @type {import('webpack').Plugin}
     *
     */
    const createSymLinkPlugin = new (class {
      apply(compiler) {
        compiler.hooks.afterEmit.tapPromise("SymlinkWebpackPlugin", async (compiler) => {
          if (isServer) {
            const from = join(compiler.options.output.path, "../static");
            const to = join(compiler.options.output.path, "static");

            try {
              await access(from);
              console.log(`${from} already exists`);
              return;
            } catch (error) {
              if (error.code === "ENOENT") {
                // No link exists
              } else {
                throw error;
              }
            }

            await symlink(to, from, "junction");
            console.log(`created symlink ${from} -> ${to}`);
          }
        });
      }
    })()

    config.plugins.push(createSymLinkPlugin);
    return config;
  },
};

export default nextConfig;
