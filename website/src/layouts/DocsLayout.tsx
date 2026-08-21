import { NavLink, Outlet } from "react-router-dom";
import { SiteFooter, SiteHeader } from "../components/SiteChrome";

const sections = [
  { to: "/docs", label: "Introduction", end: true },
  { to: "/docs/how-it-works", label: "How it works" },
  { to: "/docs/getting-started", label: "Getting started" },
  { to: "/docs/desktop-client", label: "Desktop client" },
  { to: "/docs/sync-protocol", label: "Sync protocol" },
  { to: "/docs/api", label: "API reference" },
  { to: "/docs/security", label: "Security" },
];

export function DocsLayout() {
  return (
    <div className="min-h-screen">
      <SiteHeader />
      <div className="mx-auto grid max-w-6xl gap-10 px-6 py-12 lg:grid-cols-[220px_minmax(0,1fr)]">
        <aside className="lg:sticky lg:top-24 lg:self-start">
          <p className="mb-4 text-xs font-semibold uppercase tracking-[0.16em] text-[color:var(--color-muted)]">
            On this site
          </p>
          <nav className="flex flex-col gap-1 text-sm">
            {sections.map((section) => (
              <NavLink
                key={section.to}
                to={section.to}
                end={section.end}
                className={({ isActive }) =>
                  [
                    "rounded-md px-3 py-2 transition-colors",
                    isActive
                      ? "bg-[color:var(--color-accent-soft)] font-medium text-[color:var(--color-accent)]"
                      : "text-[color:var(--color-muted)] hover:bg-white hover:text-[color:var(--color-ink)]",
                  ].join(" ")
                }
              >
                {section.label}
              </NavLink>
            ))}
          </nav>
        </aside>
        <main>
          <Outlet />
        </main>
      </div>
      <SiteFooter />
    </div>
  );
}
