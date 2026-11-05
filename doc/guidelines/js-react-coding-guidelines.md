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

## Architecture (`src/work-pulse-app/src`)
```text
index.html -> main.jsx
               -> React providers + BrowserRouter
               -> app/App.jsx (shell + routes)
                    -> pages/*.jsx (route workflows)
                         -> hooks/*.jsx (shared activity/category API state)
                         -> config/api.js + Axios -> Rust REST API (/api/v1)
                    -> components/*.jsx (shared UI)
                    -> lib/*.js (utilities)
```

- `main.jsx` mounts the app inside `React.StrictMode`, MUI Joy's `CssVarsProvider` and `CssBaseline`, and
  `BrowserRouter`. Keep global providers here.
- `app/App.jsx` owns the application shell, navigation, and route table. Route pages own their workflow and
  page-specific state.
- Use `hooks/` for reusable API-backed state and operations, as with `useActivities` and `useCategories`. Keep
  one-off page workflows in their page unless they become reusable.
- Configure the service base URL through `config/api.js` (`VITE_API_BASE_URL`, defaulting to
  `http://localhost:8080`) and use Axios for HTTP requests to the Rust service. The browser app requires that service
  to be running; Vite serves only the frontend.
- Keep API calls out of shared components. Pass data and event handlers through props; components may use routing
  when their UI needs navigation.
- Keep state local to the page or hook that owns the workflow, and pass it through props. Do not add a global state
  library or context unless a concrete cross-route sharing need justifies it.
- Follow the existing separate `loading` and `error` state pattern in the data hooks. Guard results from overlapping
  requests when an older response could overwrite newer state.
- Show useful errors in the UI and retain diagnostic details in logs or operation results where appropriate.

### Adding a feature
1. Agree on the REST endpoint and request/response payload with the backend service under
   `src/work-pulse-service/` and the relevant core use case.
2. Put reusable activity/category API operations in the corresponding hook; keep page-specific calls and state in
   the owning page.
3. Add or extend a component under `components/` and pass data and callbacks through props; keep HTTP calls out of
   shared components.
4. Add or extend frontend tests using the existing Jest setup.
5. Update `doc/architecture/` documentation if application boundaries or data flow change.

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
