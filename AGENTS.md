# Repository guidance

## Common commands

Day-to-day work goes through `just`; recipes live in `Justfile`. After installing `just`, run `just once` for the
one-time setup (`enable-wasm` and `install-tools`).

| Task | Command |
| --- | --- |
| Format | `just fmt` |
| Type-check native targets | `just check` |
| Type-check `wasm32-unknown-unknown` | `just check-wasm` |
| Clippy with warnings denied | `just clippy` |
| Unit tests only | `just test-lib` |
| Doc tests only | `just test-doc` |
| Native test bundle | `just test` |
| Build documentation with warnings denied | `just doc` |
| Main pre-PR verification bundle | `just verify` |

`just verify` runs format checking, native and WASM checks, Clippy, native tests (including doc tests), and the
warning-denied documentation build. CI additionally runs a locked release build, pins an MSRV check to Rust 1.89.0,
and has a blocking `cargo-deny` job. `cargo-semver-checks` remains non-blocking until version 0.1.0 provides a registry
baseline. CI sets `RUSTFLAGS=-D warnings` globally and `RUSTDOCFLAGS=-D warnings` for documentation, so new compiler
and Rustdoc warnings fail their respective blocking jobs.

## Architecture

This is the single-crate `leptos-css` library. `Cargo.toml` is the source of truth for the development-only files
excluded from the published package.

### Module purity

A pure module owns exactly one concept. Code, docs, and tests respect that ownership; downstream callers reason
against the module's contract rather than its implementation.

#### Structural purity

1. One concept per module, named by what the concept is. Prefer nouns; `utils`, `helpers`, `common`, `shared`, and
   `misc` signal missing ownership rather than a coherent concept.
2. Put invariants at the boundary that enforces them. Validation belongs in the validated type's constructor. Once a
   value crosses a module boundary, downstream code should trust the invariant instead of re-validating it.
3. Default to the narrowest visibility that works. Use `pub(crate)` by default and `pub` only for the published,
   versioned surface.
4. Give each piece of logic one home. Shared operations belong to the module that owns the type they operate on.

#### Documentary purity

Doc comments describe a module's own types and functions in that module's vocabulary. They do not name callers in an
upper layer, expose lower-layer implementation details, or explain how another module consumes their outputs.

1. State the module contract on the first line of its module-level documentation.
2. Reference downward or sideways, not upward: a module may name direct collaborators it imports, but not modules
   that import it.
3. Do not leak a callee's internals into caller-facing documentation.
4. Public top-level types are shared vocabulary and may be referenced across layers when useful.
5. These rules apply to Rust doc comments (`///` and `//!`), not implementation comments (`//`) or tested panic/error
   strings.

### Module layout (`src/`)

`lib.rs` re-exports the typed values from `value`, plus `CheckedDeclaration` and `PropertyName`. The crate root is the
canonical import path for shared values and stored declarations; property selectors live under
`leptos_css::property`.

- `value.rs` owns typed CSS primitives and closed grammars, including `CssValue`, `CssLength`, `CssDimension`,
  `CssDimensionExpr`, colors, angles, times, `FontWeight`, box-model values, typed custom properties, `var()`, and CSS
  math helpers. `CssWriteTo` is an open, serialization-only, zero-intermediate-allocation write path to caller-owned
  buffers. The sealed `CheckedCssValue` marker limits checked declarations to crate-validated grammars; each new
  concrete checked grammar must be deliberately added to the private display-and-sealing allow-list after its
  constructors and serialization are validated. Token validity, numeric invariants, and value formatting belong here
  rather than in higher layers.
- `declaration.rs` owns the unforgeable `CheckedDeclaration` boundary and its direct Leptos `IntoStyle` adapter. A
  declaration retains its checked property/value pairing even when stored heterogeneously.
- `property.rs` owns the `#[non_exhaustive]` `PropertyName` catalog and maps variants to kebab-case CSS names with
  `as_str()`. Extending the catalog requires a variant here; adding checked declaration support also requires the
  property's exact grammar and a sealed selector. `PropertyName` variants are intentionally exempt from individual
  missing-doc warnings because their names are the documentation; do not generalize that exemption. The
  crate-private `typed_property!` macro generates selectors such as `TouchActionProperty`, whose `declare` method
  accepts exactly that property's grammar and returns `CheckedDeclaration`.

### Invariants worth preserving

- Floating-point constructors such as `px`, `pct`, `deg`, `s`, and `hsl` reject invalid input at construction time,
  before a value can reach rendered CSS. Panicking convenience constructors reject non-finite or out-of-range values;
  use their fallible `try_*` counterparts for runtime-derived input.
- `CssDimension` models only `<length-percentage>`. Grammars that permit `auto`, such as margin and inset, use
  `LengthPercentageAuto`; grammars that require non-negative direct values, such as padding and gap, use the
  non-negative wrapper types. Range-restricted calculations remain valid because CSS clamps them after evaluation.
  Do not weaken these property-specific constraints or reject calculations based on their unevaluated terms.
- `CssCustomProperty<T>` validates a `--`-prefixed name and fixes one grammar `T`. `var()` accepts only a typed custom
  property plus a matching mandatory fallback. Do not add raw-name or fallback-free constructors to the checked API.
- Preserve the checked boundary's concrete escape-hatch restrictions: `CssValue` has no arbitrary-string variant,
  typed primitives do not implement Leptos `IntoStyleValue`, `PropertyName` does not implement `AsRef<str>`, and
  `CheckedDeclaration` has no public raw constructor. Implementing the open `CssWriteTo` trait must not grant access
  to `CheckedCssValue` or checked declarations.
- Keep every public enum `#[non_exhaustive]`. Downstream-facing matches and examples must include a wildcard arm.
- Public items must retain doc comments. `missing_docs = "warn"` becomes an error in the warning-denied Rustdoc CI job.
- `clippy::pedantic` is denied crate-wide. The allow-list in `Cargo.toml` is deliberate; extend it only when a warning
  is genuinely noise, and otherwise satisfy the lint locally.

### Compatibility constraints

- The MSRV is Rust 1.89.0 and is enforced in CI. Do not use newer standard-library APIs or language features unless
  the MSRV is intentionally bumped in `Cargo.toml`, the README, and CI together.
- `Cargo.toml` currently requires `leptos = "0.8.19"` with default features disabled. `CheckedDeclaration` uses Leptos
  Tachys style-rendering primitives for its adapter.
- Coordinate Leptos dependency and `nightly` feature changes with `leptos-styles` so the related crates remain in
  lockstep and do not introduce duplicate Leptos versions into downstream lockfiles.

## CSS support policy

- Target modern CSS. Partial property coverage is intentional; never claim that the crate covers every property in
  the CSS property index.
- Treat property-specific selector APIs as the checked declaration surface. `PropertyName` is only a curated property-name
  catalog, and `CssValue` is only a heterogeneous primitive-value container.
- Keep checked APIs fail-closed: do not accept arbitrary property names, value strings, function bodies, variable
  fallbacks, or unchecked identifier strings.
- Support custom properties and `var()` as common, first-class CSS features, but keep their names, assigned values,
  references, and fallbacks typed and validated.
- Do not add legacy aliases or deprecated properties merely for index completeness.
- Do not add speech or aural properties, SVG-specific paint properties, or other niche property families without an
  explicit product requirement.
- Prefer finishing a supported property's real grammar and its closely related logical/physical family over adding a
  broad list of property names.
- Cite the normative specification in the Rustdoc for each property grammar. The Rust property selectors and grammar
  types are the source of truth; do not maintain a parallel coverage manifest unless it drives generation or tests.

### Standards baseline

The catalog admits a property only when it is the canonical, non-deprecated name for a general-purpose authoring
feature and either appears in the [CSS Snapshot 2026 property index](https://www.w3.org/TR/css-2026/#properties) or
comes from an explicitly selected modern module. The catalog and public value APIs draw on these modern modules:

- [CSS Basic User Interface Level 4](https://www.w3.org/TR/css-ui-4/) for `appearance`, `pointer-events`, and
  `user-select`;
- [CSS Sizing Level 4](https://www.w3.org/TR/css-sizing-4/) for `aspect-ratio`;
- [Filter Effects Level 2](https://drafts.csswg.org/filter-effects-2/) for `backdrop-filter`;
- [CSS Overflow Level 3](https://www.w3.org/TR/css-overflow-3/) for `overflow-x`, `overflow-y`, `scroll-behavior`, and
  `scrollbar-gutter`;
- [CSS Overscroll Behavior Level 1](https://www.w3.org/TR/css-overscroll-1/) for the `overscroll-behavior` family;
- [CSS Text Decoration Level 4](https://www.w3.org/TR/css-text-decor-4/) for `text-decoration-skip-ink`,
  `text-decoration-thickness`, and `text-underline-offset`;
- [Pointer Events Level 4](https://www.w3.org/TR/pointerevents4/) for `touch-action`;
- [CSS Values Level 4](https://www.w3.org/TR/css-values-4/) for typed math expressions and modern dimensions;
- [CSS Environment Variables Level 1](https://www.w3.org/TR/css-env-1/) for typed `env()` expressions;
- [CSS Custom Properties Level 1](https://www.w3.org/TR/css-variables-1/) for typed custom properties and `var()`; and
- [CSS View Transitions Level 1](https://www.w3.org/TR/css-view-transitions-1/) for `view-transition-name`.

Legacy aliases, speech/aural properties, SVG-specific paint and rendering properties, and other niche families are
intentionally not targets. Browser support is separate from grammar validity.
