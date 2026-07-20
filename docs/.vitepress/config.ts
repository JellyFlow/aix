import { defineConfig } from "vitepress";

function normalizeBasePath(input: string | undefined): string {
  if (!input || input === "/") {
    return "/";
  }

  const withLeadingSlash = input.startsWith("/") ? input : `/${input}`;
  return withLeadingSlash.endsWith("/")
    ? withLeadingSlash
    : `${withLeadingSlash}/`;
}

const runtimeProcess = globalThis as typeof globalThis & {
  process?: { env?: Record<string, string | undefined> };
};

const base = normalizeBasePath(runtimeProcess.process?.env?.BASE_PATH ?? "/aix");

export default defineConfig({
  title: "AIX",
  description: "Official documentation and package lab for the AIX file format.",
  base,
  appearance: false,
  lastUpdated: true,
  cleanUrls: true,
  head: [
    ["meta", { name: "theme-color", content: "#f4f1e9" }],
    ["meta", { property: "og:title", content: "AIX" }],
    [
      "meta",
      {
        property: "og:description",
        content: "AIX is a file format for package structure, page schema, and tool surfaces."
      }
    ]
  ],
  themeConfig: {
    nav: [
      { text: "Format", link: "/format" },
      { text: "Docs", link: "/format" },
      { text: "Packages", link: "/packages" },
      { text: "Playground", link: "/playground" },
      { text: "GitHub", link: "https://github.com/jsar-project/aix" }
    ],
    sidebar: [
      {
        text: "AIX",
        items: [
          { text: "Home", link: "/" },
          { text: "Format", link: "/format" },
          { text: "Packages", link: "/packages" },
          { text: "Playground", link: "/playground" }
        ]
      }
    ],
    docFooter: {
      prev: "Previous",
      next: "Next"
    },
    footer: {
      message: "AIX file format documentation and package lab.",
      copyright: "Released for the jsar-project/aix repository."
    },
    socialLinks: [
      { icon: "github", link: "https://github.com/jsar-project/aix" }
    ],
    outline: "deep",
    search: {
      provider: "local"
    }
  }
});
