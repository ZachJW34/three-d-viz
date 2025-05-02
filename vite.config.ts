import { defineConfig, HtmlTagDescriptor, Plugin } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "node:path";
import fs from "node:fs/promises";
import { viteStaticCopy } from "vite-plugin-static-copy";
import tailwindcss from "@tailwindcss/vite";
import { parse } from "node-html-parser";

const DIRNAME = import.meta.dirname;

const htmlPlugin = (): Plugin => ({
  name: "trunk-transform",
  async transformIndexHtml(html) {
    const bevyIndexHtmlPath = path.join(DIRNAME, "trunk", "bevy-index.html");
    const content = (await fs.readFile(bevyIndexHtmlPath)).toString("utf-8");
    const root = parse(content);
    const links = root.querySelectorAll("link");
    const scripts = root.querySelectorAll("script");
    const tags: HtmlTagDescriptor[] = [
      ...links.map<HtmlTagDescriptor>((link) => ({
        tag: "link",
        attrs: link.attributes,
        injectTo: "head",
      })),
      ...scripts.map<HtmlTagDescriptor>((script) => ({
        tag: "script",
        attrs: script.attributes,
        children: script.innerText,
        injectTo: "body",
      })),
    ];

    return {
      html,
      tags,
    };
  },
});

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    htmlPlugin() as any,
    vue(),
    tailwindcss(),
    viteStaticCopy({
      targets: [
        {
          src: ["./trunk/**", "!./trunk/.stage/**"],
          dest: "assets",
        },
      ],
    }),
  ],
  server: {
    fs: {
      strict: false,
    },
  },
});
