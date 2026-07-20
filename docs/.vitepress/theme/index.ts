import DefaultTheme from "vitepress/theme";
import type { Theme } from "vitepress";
import HomeExperience from "./components/HomeExperience.vue";
import "./style.css";

const theme: Theme = {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("HomeExperience", HomeExperience);
  }
};

export default theme;
