import path from 'path';
import { defaultTheme } from "@vuepress/theme-default";
import { defineUserConfig } from "vuepress/cli";
import { webpackBundler } from '@vuepress/bundler-webpack'
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin"; 
import { palettePlugin } from '@vuepress/plugin-palette'

/** @type {import('vite').UserConfig} */
export default defineUserConfig({
  head: [
    ['link', { rel: 'icon', href: '/spritzer/favicon.ico' }],
    ['link', { rel: 'stylesheet', href: "https://unpkg.com/splitting/dist/splitting.css" }],
    ['link', { rel: 'stylesheet', href: '/spritzer/base.css' }],
    ['script', { src: 'https://unpkg.com/splitting/dist/splitting.min.js' }],
  ],
  base: '/spritzer/',
  language: 'en-US',
  title: "Spritzer",
  description: "ALTTP Sprite + Dungeon Randomizer",
  plugins: [
    palettePlugin({
      preset: 'sass',
      userStyleFile: '.vuepress/index.scss',
    }),
    {
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
      "/guide",
      "/randomize"
    ],
  }),
  bundler: webpackBundler({
    postcss: {},
    vue: {},
    scss: {
      sassOptions: {
        css: {
          url: false,
        }
      }
    },
    configureWebpack(config) {
      config.infrastructureLogging = {
        colors: true,
        appendOnly: true,
        level: 'verbose',
      };
      config.experiments = { asyncWebAssembly: true };
      config.plugins.push(new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, process.cwd() + "/docs"),
        args: "--log-level info",
        extraArgs: '--no-typescript',
        outDir: path.resolve(__dirname, "./pkg"),
        pluginLogLevel: 'info',
      }));
    }
  }),
});
