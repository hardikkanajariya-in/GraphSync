import type { ReactNode } from "react";

interface DocArticleProps {
  title: string;
  lede: string;
  children: ReactNode;
}

export function DocArticle({ title, lede, children }: DocArticleProps) {
  return (
    <article>
      <header className="mb-10 border-b border-[color:var(--color-line)] pb-8">
        <p className="text-sm font-medium uppercase tracking-[0.18em] text-[color:var(--color-muted)]">
          GraphSync docs
        </p>
        <h1 className="mt-3 font-serif text-4xl tracking-tight sm:text-5xl">{title}</h1>
        <p className="mt-4 max-w-2xl text-lg leading-relaxed text-[color:var(--color-muted)]">
          {lede}
        </p>
      </header>
      <div className="doc-body max-w-3xl">{children}</div>
    </article>
  );
}
