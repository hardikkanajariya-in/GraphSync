import { Link, NavLink } from "react-router-dom";
import { AUTHOR, LINKS, PRODUCT } from "../lib/site";

export function SiteHeader() {
  return (
    <header className="sticky top-0 z-50 border-b border-[color:var(--color-line)] bg-[color:var(--color-paper)]/95 backdrop-blur-sm">
      <div className="mx-auto flex max-w-6xl items-center justify-between px-6 py-4">
        <Link to="/" className="group flex items-baseline gap-2">
          <span className="text-lg font-semibold tracking-tight">{PRODUCT.name}</span>
          <span className="hidden text-sm text-[color:var(--color-muted)] sm:inline">
            API
          </span>
        </Link>

        <nav className="flex items-center gap-6 text-sm">
          <NavLink
            to="/docs"
            className={({ isActive }) =>
              isActive
                ? "font-medium text-[color:var(--color-ink)]"
                : "text-[color:var(--color-muted)] hover:text-[color:var(--color-ink)]"
            }
          >
            Docs
          </NavLink>
          <a
            href={LINKS.releases}
            className="text-[color:var(--color-muted)] hover:text-[color:var(--color-ink)]"
          >
            Download client
          </a>
          <Link
            to="/docs/getting-started"
            className="rounded-full bg-[color:var(--color-ink)] px-4 py-2 font-medium text-white hover:opacity-90"
          >
            Get started
          </Link>
        </nav>
      </div>
    </header>
  );
}

export function SiteFooter() {
  return (
    <footer className="border-t border-[color:var(--color-line)]">
      <div className="mx-auto flex max-w-6xl flex-col gap-6 px-6 py-12 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <p className="font-serif text-2xl">{PRODUCT.name}</p>
          <p className="mt-2 max-w-md text-sm leading-relaxed text-[color:var(--color-muted)]">
            Built for people who want sync without giving up ownership of their files.
          </p>
        </div>

        <div className="text-sm text-[color:var(--color-muted)]">
          <p>
            {AUTHOR.role}:{" "}
            <a
              href={AUTHOR.site}
              className="font-medium text-[color:var(--color-ink)] underline decoration-[color:var(--color-line)] underline-offset-4 hover:decoration-[color:var(--color-accent)]"
            >
              {AUTHOR.name}
            </a>
          </p>
          <p className="mt-2">
            <a href={AUTHOR.site} className="hover:text-[color:var(--color-ink)]">
              hardikkanajariya.in
            </a>
          </p>
        </div>
      </div>
    </footer>
  );
}
