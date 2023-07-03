import { defineConfig } from "cypress";
import { config as dotenv } from "dotenv";

let env = {};
dotenv({ processEnv: env, path: "./.env" });
dotenv({ processEnv: env, path: "./.env.local" });

export default defineConfig({
  env,
  e2e: {
    baseUrl: "http://localhost:5173",
    setupNodeEvents(on, config) {
      // implement node event listeners here
    },
  },
});
