import path from 'path';
import { defaultTheme } from "@vuepress/theme-default";
import { defineUserConfig } from "vuepress/cli";
import { webpackBundler } from '@vuepress/bundler-webpack'
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

/** @type {import('vite').UserConfig} */
export default defineUserConfig({
  head: [
    ['link', { rel: 'icon', href: '/spritzer/favicon.ico' }],
  ],
  base: '/spritzer/',
  language: 'en-US',
  title: "Spritzer",
  description: "Zelda: A Link to the Past Dungeon Sprite + Dungeon Randomizer",
  plugins: [{
    '@vuepress/medium-zoom': {
      selector: 'img.zoom-custom-imgs',
    }
  }],
  theme: defaultTheme({
    colorMode: 'light',
    colorModeSwitch: false,
    contributors: false,
    editLink: false,
    lastUpdated: false,
    logo: "/images/logo.png",
    repo: "notoriousb1t/spritzer",
    sidebar: false,
    navbar: [
      "/get-started",
      "/randomize"
    ],
  }),
  bundler: webpackBundler({
    postcss: {},
    vue: {},
    scss: {},
    configureWebpack(config, isServer, isBuild) {
      config.infrastructureLogging = {
        colors: true,
        appendOnly: true,
        level: 'verbose',
      };

      config.experiments = { asyncWebAssembly: true };

      if (!isBuild && !isServer) {
        config.plugins.push(new WasmPackPlugin({
          crateDirectory: path.resolve(__dirname, process.cwd() + "/docs"),
          args: "--log-level info",
          extraArgs: '--no-typescript',
          // forceMode: "production",
          forceWatch: true,
          outDir: path.resolve(__dirname, "./pkg"),
          pluginLogLevel: 'info',
        }));
      }
    }
  }),
});
