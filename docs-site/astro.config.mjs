import { execSync } from "node:child_process";
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import starlightOpenAPI, { openAPISidebarGroups } from "starlight-openapi";

const GITHUB_URL = "https://github.com/spxrogers/oura-toolkit";

// The commit the live site was built from — surfaced in the footer as a "what's currently
// live" breadcrumb (see src/components/LastUpdated.astro). Prefer the CI-provided SHA (GitHub
// Actions sets GITHUB_SHA; other hosts set their own), then fall back to the local git HEAD for
// `just docs-build`/`docs-dev` and out-of-band local builds. Empty string if neither resolves
// (the footer then hides the hash entirely).
function resolveCommitSha() {
  const fromEnv =
    process.env.GITHUB_SHA ||
    process.env.COMMIT_SHA ||
    process.env.VERCEL_GIT_COMMIT_SHA ||
    process.env.CF_PAGES_COMMIT_SHA;
  if (fromEnv) return fromEnv.trim();
  try {
    return execSync("git rev-parse HEAD", { encoding: "utf8" }).trim();
  } catch {
    return "";
  }
}
const commitSha = resolveCommitSha();

// https://astro.build/config
export default defineConfig({
  // Apex domain served by GitHub Pages (public/CNAME). No `base`: the site is at the root.
  site: "https://ouratoolkit.com",
  // Inline the build-time commit SHA + the repo URL (single-sourced from GITHUB_URL above, so
  // the commit link can't drift from the rest of the site) for the LastUpdated footer override.
  vite: {
    define: {
      "import.meta.env.PUBLIC_COMMIT_SHA": JSON.stringify(commitSha),
      "import.meta.env.PUBLIC_REPO_URL": JSON.stringify(GITHUB_URL),
    },
  },
  integrations: [
    starlight({
      title: "oura-toolkit",
      description:
        "Your Oura Ring data everywhere you work: a fast Rust CLI, a local MCP server, a Claude plugin, and SDKs in six languages — all driven by Oura's OpenAPI spec.",
      logo: { src: "./src/assets/logo.svg", replacesTitle: false },
      favicon: "/favicon.svg",
      // "Midnight" theme: self-hosted fonts (no external font requests) + the palette,
      // hero, and chrome styling in theme.css.
      customCss: [
        "@fontsource-variable/manrope",
        "@fontsource/ibm-plex-mono/400.css",
        "@fontsource/ibm-plex-mono/600.css",
        "./src/styles/theme.css",
      ],
      // One dark code theme in BOTH color schemes: terminals are screens, not paper —
      // the docs are CLI-output-heavy and the night-navy frames carry the brand.
      expressiveCode: {
        themes: ["night-owl"],
        styleOverrides: {
          borderRadius: "0.6rem",
          borderColor: "#24304f",
          codeFontFamily: "'IBM Plex Mono', ui-monospace, monospace",
          frames: { shadowColor: "transparent" },
        },
      },
      head: [
        {
          // Social-preview card (public/og.png) — the "Midnight" brand card, rendered
          // from the site's own self-hosted fonts.
          tag: "meta",
          attrs: { property: "og:image", content: "https://ouratoolkit.com/og.png" },
        },
        {
          // Dark is the flagship ("Midnight"): seed the theme choice to dark for
          // first-time visitors. Runs before Starlight's ThemeProvider script (config
          // head entries render earlier in <head>), and never overrides an explicit
          // user choice since it only writes when the key is absent.
          tag: "script",
          content:
            "try{if(!localStorage.getItem('starlight-theme'))localStorage.setItem('starlight-theme','dark')}catch(e){}",
        },
      ],
      social: [{ icon: "github", label: "GitHub", href: GITHUB_URL }],
      editLink: { baseUrl: `${GITHUB_URL}/edit/main/docs-site/` },
      lastUpdated: true,
      components: {
        // Append the build/deploy commit hash to Starlight's "Last updated" footer.
        LastUpdated: "./src/components/LastUpdated.astro",
      },
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
