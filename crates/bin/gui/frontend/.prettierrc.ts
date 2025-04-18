/**
 *  .prettierrc.ts
 *
 *  https://prettier.io/docs/configuration
 */
import type { Config } from "prettier";

const config: Config = {
  plugins: ["prettier-plugin-svelte"],
  semi: true,
};

export default config;
