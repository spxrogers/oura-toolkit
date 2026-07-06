import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import starlightOpenAPI, { openAPISidebarGroups } from "starlight-openapi";

const GITHUB_URL = "https://github.com/spxrogers/oura-toolkit";

// https://astro.build/config
export default defineConfig({
  // Apex domain served by GitHub Pages (public/CNAME). No `base`: the site is at the root.
  site: "https://ouratoolkit.com",
  integrations: [
    starlight({
      title: "oura-toolkit",
      description:
        "Your Oura Ring data everywhere you work: a fast Rust CLI, a local MCP server, a Claude plugin, and SDKs in six languages — all driven by Oura's OpenAPI spec.",
      logo: { src: "./src/assets/logo.svg", replacesTitle: false },
      favicon: "/favicon.svg",
      social: [{ icon: "github", label: "GitHub", href: GITHUB_URL }],
      editLink: { baseUrl: `${GITHUB_URL}/edit/main/docs-site/` },
      lastUpdated: true,
      sidebar: [
        {
          label: "Guides",
          items: [{ autogenerate: { directory: "guides" } }],
        },
        {
          label: "Concepts",
          items: [{ autogenerate: { directory: "concepts" } }],
        },
        {
          label: "CLI",
          items: [{ autogenerate: { directory: "cli" } }],
        },
        {
          label: "SDKs",
          items: [{ autogenerate: { directory: "sdks" } }],
        },
        ...openAPISidebarGroups,
      ],
      plugins: [
        starlightOpenAPI([
          {
            base: "api",
            label: "API Reference",
            // The docs spec (produced by `just docs-spec` from the OVERLAID spec, NOT the
            // pristine one): the overlay rewrites the leaked `api.None.com` server URL to
            // api.ouraring.com and cleans the models, then the docs step normalizes the spec's
            // `x-codeSamples` language labels to Shiki grammar ids so the request samples get
            // syntax-highlighted. Path is relative to this project root (docs-site/); `just
            // docs-build`/`docs-dev` produce this derived file first.
            schema: "../codegen/build/openapi.docs.json",
            collapsed: true,
          },
        ]),
      ],
    }),
  ],
});
