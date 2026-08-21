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

Import the repository in Vercel and deploy from the **repository root**.

The root [`vercel.json`](../vercel.json) handles install, build, output directory, and SPA routing automatically. No separate root directory setting is required.
