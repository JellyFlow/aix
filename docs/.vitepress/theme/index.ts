import DefaultTheme from "vitepress/theme";
import type { Theme } from "vitepress";
import HomeExperience from "./components/HomeExperience.vue";
import PackageLab from "./components/PackageLab.vue";
import "./style.css";

const theme: Theme = {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("HomeExperience", HomeExperience);
    app.component("PackageLab", PackageLab);
  }
};

export default theme;
