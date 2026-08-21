# GraphSync website

Product landing page and documentation for the GraphSync coordination API.

- **Author & maintainer:** [Hardik Kanajariya](https://hardikkanajariya.in)

This package is part of the root pnpm workspace. Dependencies are installed once at the repository root.

## Local development

From the repository root:

```bash
pnpm install
pnpm dev:website
```

Or from this directory:

```bash
pnpm dev
```

## Deploy to Vercel

Deploy to Vercel from the **repository root** (leave Root Directory empty in project settings).

The root [`vercel.json`](../vercel.json) runs:

```bash
pnpm install
pnpm --filter graphsync-website build
```

If your Vercel project uses **Root Directory = `website`**, the local [`vercel.json`](vercel.json) in this folder is used instead.
