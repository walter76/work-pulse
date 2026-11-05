# Coding Guidelines for JavaScript / React

These guidelines apply to the frontend in `src/`. **MUST** indicates a hard rule; **SHOULD** is the default and may be
deviated from when there is a clear reason.

## Commands
Run npm commands from the repository root.

```text
npm run dev          # Vite dev server, http://localhost:1420, no Rust bridge
npm run tauri:dev    # desktop application with hot reload
npm test             # vitest run
npm run build        # production frontend build
```

See `README.md` for prerequisites and packaging.

## Tooling
- **JavaScript, not TypeScript.** Components and tests use `.jsx`; plain modules use `.js`.
- ES modules only (`"type": "module"` in `package.json`).
- Vite 7 with `@vitejs/plugin-react`; Vitest 5 with `jsdom` and `@testing-library/react`.
- There is no linter or formatter configuration yet. Match the existing style by hand (see Style).

## Style
- Single quotes, no semicolons, trailing commas in multi-line literals, 2-space indent, 100-character lines.
- Function components declared with `function Name(props) {}` and exported with `export default Name` at the end of
  the file. One component per file, file name equals component name.
- **MUST** add a JSDoc header to each component describing its purpose, input props with `@param` tags, and rendered
  return value with an `@returns` tag.
- Destructure props in the parameter list and give optional arrays a default (`availableBundles = []`).
- **SHOULD** prefer `const` arrow functions for handlers and helpers inside components.
- **SHOULD** prefer `??` and optional chaining over manual null checks; prefer spread over mutation when deriving state.
- **MUST NOT** mutate state objects or arrays in place; always produce new values.
- Comments must not merely restate what the code does.

## Naming
- Components: `UpperCamelCase`. Hooks and helpers: `camelCase`. Event-handler props: `onVerb` (`onSave`, `onCancel`,
  `onOpenRepository`); their implementations in the parent: `handleVerb` or a descriptive verb (`openRepository`).
- Keep the JSON contract names from the backend unchanged in frontend data (`isExternalDelivery`, `repository_bundles`,
  `mapping_error`). Do not rename fields when passing backend payloads through the UI.

## Architecture (`src`)
```text
main.jsx -> App.jsx -> components/*.jsx
                 \-> @tauri-apps/api/core invoke -> Rust commands
```

- `main.jsx` mounts `App` inside `React.StrictMode` and the MUI Joy `CssVarsProvider`. Keep global providers here.
- `App.jsx` is the only module that imports `invoke`. It owns all application state (mapping, scan, repository,
  editing device), performs backend calls, and passes data and callbacks down.
- `components/` contains presentational components. They receive data and callbacks through props and hold only local
  UI state (for example the edit draft in `DeviceEditForm`).
- **MUST NOT** call `invoke` from a component in `components/`. Lift the call into `App.jsx` and pass a callback.
- **MUST NOT** introduce a global state library or context for the current scope; prop passing from `App.jsx` is
  sufficient.
- Async state follows the shape `{ status: 'idle' | 'loading' | 'loaded' | 'error', data, error }`. Reuse this shape
  for new backend-backed state instead of separate boolean flags.
- **MUST** guard async results that can be superseded with the request-ID pattern in `App.jsx`
  (`mappingRequestIdRef`): capture the ID before awaiting, discard the result if the ID has moved on.
- Backend errors arrive as strings. Store them with `String(error)` and display them; do not swallow them.
- Browser-only mode (`npm run dev`) has no `invoke`. Do not add code paths that depend on the browser mode working;
  it exists for layout work only.

### Adding a feature
1. Agree the command name and payload with the backend (`src-tauri/src/commands.rs`, `models.rs`).
2. Add state and the `invoke` call to `App.jsx`, following the async-state shape and request-ID guard where relevant.
3. Add or extend a component under `components/`; keep it free of `invoke` and backend knowledge beyond the payload
   shape.
4. Add or extend tests (see Testing). Mock `invoke` for the new command.
5. Update `docs/architecture/architecture.md` if state ownership or data flow changes.

## UI (MUI Joy)
- Use `@mui/joy` components; do not add `@mui/material` or another component library.
- Prefer the `sx` prop and Joy theme tokens for spacing and sizing. Keep `src/styles.css` for app-shell layout only.
- Use semantic roles and labels (`Button` text, `FormLabel`, `Checkbox label`) so tests can query by role and name.
- Dialogs use `Modal` + `ModalDialog` + `ModalClose`, controlled by the parent via `open` and `onClose`.

## Testing
- Test files are co-located with the module under test and named `<Module>.test.jsx`.
- Vitest with `environment: 'jsdom'`; `src/test/setup.js` registers `@testing-library/jest-dom/vitest` matchers.
- **MUST** mock the Tauri bridge in every test that renders `App`:

  ```js
  const mockInvoke = vi.fn()
  vi.mock('@tauri-apps/api/core', () => ({ invoke: (...args) => mockInvoke(...args) }))
  beforeEach(() => mockInvoke.mockReset())
  afterEach(() => cleanup())
  ```

  Dispatch on the command name inside `mockInvoke.mockImplementation((command, args) => …)` and return
  `Promise.resolve(payload)` for known commands. Fail loudly on unknown commands rather than silently resolving, so a
  renamed command cannot pass by accident.
- Use payloads that match the Rust models exactly (`RepositoryOpen`, `BundleScan`, `MedicalDevicesMappingFile`).
- Query with `screen.getByRole` / `getByLabelText`; avoid test IDs and CSS selectors.
- Use `waitFor` for state that follows an awaited `invoke`; use deferred promises (`new Promise((resolve) => …)`) to
  test ordering and stale-request behavior.
- **SHOULD** assert what the backend received (`expect(mockInvoke).toHaveBeenCalledWith('save_mapping', { mapping })`)
  in addition to what the UI shows.
- Presentational components may be tested directly without mocking `invoke`.
- Tests must not depend on the desktop runtime, the filesystem, or `data/md_mapping.json`.

## Dependencies
- Frontend dependencies belong in `package.json`; keep Rust-only concerns out of it.
- **SHOULD** check whether React, MUI Joy, or `@tauri-apps/api` already provide the needed functionality before
  adding a package.
- Do not add Tauri plugins on the frontend without the matching Rust plugin registration and capability update.
