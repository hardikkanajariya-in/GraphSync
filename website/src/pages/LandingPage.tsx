import { Link } from "react-router-dom";
import { SiteFooter, SiteHeader } from "../components/SiteChrome";
import { AUTHOR, LINKS, PRODUCT } from "../lib/site";

const pillars = [
  {
    title: "Your files stay local",
    body: "GraphSync never treats your folder as cloud storage. Devices keep the actual bytes. The API only remembers what changed.",
  },
  {
    title: "Peers talk directly",
    body: "Once two devices know about a change, they transfer files over WebRTC. The API helps them find each other — it does not carry the payload.",
  },
  {
    title: "Quiet by design",
    body: "The desktop client watches the filesystem, debounces editor noise, and stays out of your way. Setup takes a folder path and an API URL. That is it.",
  },
];

const flow = [
  "You edit a file on your laptop.",
  "GraphSync notices, hashes it, and sends metadata to the API.",
  "Your other device hears about the change over WebSocket.",
  "The devices connect peer-to-peer and copy the file directly.",
];

export function LandingPage() {
  return (
    <div className="min-h-screen">
      <SiteHeader />

      <main>
        <section className="border-b border-[color:var(--color-line)]">
          <div className="mx-auto grid max-w-6xl gap-12 px-6 py-20 lg:grid-cols-[1.1fr_0.9fr] lg:items-end">
            <div>
              <p className="text-sm font-medium uppercase tracking-[0.18em] text-[color:var(--color-muted)]">
                GraphSync coordination API
              </p>
              <h1 className="mt-4 max-w-2xl font-serif text-5xl leading-[1.05] tracking-tight sm:text-6xl">
                {PRODUCT.tagline}
              </h1>
              <p className="mt-6 max-w-xl text-lg leading-relaxed text-[color:var(--color-muted)]">
                {PRODUCT.description} I built GraphSync because I wanted Logseq graphs — and
                any project folder — to follow me across machines without uploading the whole
                tree to someone else&apos;s server.
              </p>
              <div className="mt-8 flex flex-wrap gap-3">
                <Link
                  to="/docs/getting-started"
                  className="rounded-full bg-[color:var(--color-ink)] px-5 py-3 text-sm font-medium text-white hover:opacity-90"
                >
                  Read the docs
                </Link>
                <a
                  href={LINKS.releases}
                  className="rounded-full border border-[color:var(--color-line)] bg-white px-5 py-3 text-sm font-medium hover:border-[color:var(--color-ink)]"
                >
                  Download desktop app
                </a>
              </div>
            </div>

            <div className="rounded-2xl border border-[color:var(--color-line)] bg-white p-6 shadow-[0_20px_60px_rgba(21,21,21,0.06)]">
              <p className="text-xs font-semibold uppercase tracking-[0.16em] text-[color:var(--color-muted)]">
                What the API actually does
              </p>
              <ul className="mt-5 space-y-4 text-sm leading-relaxed">
                <li className="flex gap-3">
                  <span className="mt-1 h-2 w-2 rounded-full bg-[color:var(--color-accent)]" />
                  Device identity and registration
                </li>
                <li className="flex gap-3">
                  <span className="mt-1 h-2 w-2 rounded-full bg-[color:var(--color-accent)]" />
                  Sync metadata and change notifications
                </li>
                <li className="flex gap-3">
                  <span className="mt-1 h-2 w-2 rounded-full bg-[color:var(--color-accent)]" />
                  WebRTC signaling between peers
                </li>
                <li className="flex gap-3">
                  <span className="mt-1 h-2 w-2 rounded-full bg-[color:var(--color-accent)]" />
                  Manifests for catch-up after offline work
                </li>
              </ul>
              <p className="mt-6 border-t border-[color:var(--color-line)] pt-5 text-sm text-[color:var(--color-muted)]">
                What it deliberately does <strong className="text-[color:var(--color-ink)]">not</strong> do:
                store your file contents.
              </p>
            </div>
          </div>
        </section>

        <section className="mx-auto max-w-6xl px-6 py-20">
          <div className="max-w-2xl">
            <h2 className="font-serif text-4xl tracking-tight">Three ideas I kept coming back to</h2>
            <p className="mt-4 text-[color:var(--color-muted)]">
              If you are evaluating GraphSync for your team or your own workflow, these are the
              constraints that shaped the system.
            </p>
          </div>
          <div className="mt-12 grid gap-6 md:grid-cols-3">
            {pillars.map((item) => (
              <article
                key={item.title}
                className="rounded-2xl border border-[color:var(--color-line)] bg-white p-6"
              >
                <h3 className="text-lg font-semibold">{item.title}</h3>
                <p className="mt-3 text-sm leading-relaxed text-[color:var(--color-muted)]">
                  {item.body}
                </p>
              </article>
            ))}
          </div>
        </section>

        <section className="border-y border-[color:var(--color-line)] bg-white">
          <div className="mx-auto grid max-w-6xl gap-10 px-6 py-20 lg:grid-cols-2">
            <div>
              <h2 className="font-serif text-4xl tracking-tight">A sync story in four beats</h2>
              <p className="mt-4 text-[color:var(--color-muted)]">
                This is the whole product in plain language. Everything in the docs expands on
                one of these steps.
              </p>
            </div>
            <ol className="space-y-5">
              {flow.map((step, index) => (
                <li
                  key={step}
                  className="flex gap-4 rounded-xl border border-[color:var(--color-line)] p-4"
                >
                  <span className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-[color:var(--color-accent-soft)] text-sm font-semibold text-[color:var(--color-accent)]">
                    {index + 1}
                  </span>
                  <p className="text-sm leading-relaxed">{step}</p>
                </li>
              ))}
            </ol>
          </div>
        </section>

        <section className="mx-auto max-w-6xl px-6 py-20">
          <div className="rounded-3xl border border-[color:var(--color-line)] bg-[color:var(--color-accent-soft)] px-8 py-10 sm:px-12">
            <p className="text-sm font-medium uppercase tracking-[0.16em] text-[color:var(--color-accent)]">
              For developers
            </p>
            <h2 className="mt-3 max-w-2xl font-serif text-4xl tracking-tight">
              Explore the protocol, then plug in the desktop client
            </h2>
            <p className="mt-4 max-w-2xl leading-relaxed text-[color:var(--color-muted)]">
              The docs walk through authentication, event flow, signaling, and the security
              model I use in production-minded deployments. If you are implementing the API or
              integrating a client, start there.
            </p>
            <div className="mt-8 flex flex-wrap gap-3">
              <Link
                to="/docs"
                className="rounded-full bg-[color:var(--color-ink)] px-5 py-3 text-sm font-medium text-white"
              >
                Open documentation
              </Link>
              <Link
                to="/docs/api"
                className="rounded-full border border-[color:var(--color-line)] bg-white px-5 py-3 text-sm font-medium"
              >
                API reference
              </Link>
            </div>
          </div>
        </section>

        <section className="border-t border-[color:var(--color-line)]">
          <div className="mx-auto max-w-6xl px-6 py-16">
            <p className="text-sm text-[color:var(--color-muted)]">Project maintainer</p>
            <p className="mt-2 font-serif text-3xl">{AUTHOR.name}</p>
            <p className="mt-3 max-w-xl leading-relaxed text-[color:var(--color-muted)]">
              I maintain GraphSync as an open source community project. If you have questions
              about the architecture, want to run your own coordination server, or need help
              wiring the desktop client into your stack, you will find me at{" "}
              <a
                href={AUTHOR.site}
                className="font-medium text-[color:var(--color-ink)] underline decoration-[color:var(--color-line)] underline-offset-4"
              >
                hardikkanajariya.in
              </a>
              .
            </p>
          </div>
        </section>
      </main>

      <SiteFooter />
    </div>
  );
}
