import type {Metadata} from "next";

export const metadata: Metadata = {
  title: {
    default: "Next Rust Boilerplate",
    template: "%s | Next Rust Boilerplate",
  },
  description: "A Next boilerplate with Turborepo, Rust and TypeScript.",
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
    },
  },
  icons: {
    shortcut: "/favicon.ico",
  },
};