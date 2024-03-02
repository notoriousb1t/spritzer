import path from 'path';
import { defaultTheme } from "@vuepress/theme-default";
import { defineUserConfig } from "vuepress/cli";
import { webpackBundler } from '@vuepress/bundler-webpack'
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

/** @type {import('vite').UserConfig} */
export default defineUserConfig({
  lang: "en-US",
  base: '/spritzer/',
  title: "Spritzer",
  description: "Zelda: A Link to the Past Dungeon Randomizer",

  theme: defaultTheme({
    colorMode: 'dark',
    colorModeSwitch: false,
    contributors: false,
    editLink: false,
    lastUpdated: false,
    logo: "https://vuejs.press/images/hero.png",
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
    configureWebpack() {
      return {
        infrastructureLogging: {
          colors: true,
          appendOnly: true,
          level: 'verbose',
        },
        plugins: [
          new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, process.cwd() + "/docs"),
            outDir: path.resolve(__dirname, "./pkg"),
            pluginLogLevel: 'info',
            args: '--log-level warn',
            extraArgs: '--no-typescript',
          })
        ],
        experiments: {
          asyncWebAssembly: true
        }
      };
    },
  }),
});
