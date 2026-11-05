# Coding Guidelines for JavaScript / React

These guidelines apply to the frontend in `src/`. **MUST** indicates a hard rule; **SHOULD** is the default and may be
deviated from when there is a clear reason.

## Commands
Run npm commands from `src/work-pulse-app/`, the standalone frontend package.

```text
npm install
npm run dev           # Vite dev server, http://localhost:5173
npm test              # Jest tests
npm run test:watch    # Jest watch mode
npm run test:coverage # Jest coverage
npm run lint          # ESLint
npm run build         # production frontend build
```

The frontend calls the Rust API at `http://localhost:8080` by default. Set `VITE_API_BASE_URL` when it runs elsewhere.

## Tooling
- **JavaScript, not TypeScript.** Components use `.jsx`; plain modules use `.js`.
- ES modules only (`"type": "module"` in `package.json`).
- React 19, Vite 6 with `@vitejs/plugin-react`, and Jest 30 with `babel-jest` and `jsdom`.
- Use `@testing-library/react` for component tests. ESLint 9 is configured in `eslint.config.js`; formatting conventions
  are in `.prettierrc`.

## Style
- Single quotes, no semicolons, trailing commas in multi-line literals, 2-space indent, 100-character lines.
- Components are declared as `const Name = (...) => ...` and exported with `export default Name` at the end of the
  file. Keep one component per file. Component names use UpperCamelCase; component filenames use lower camel case
  (for example, `ActivitiesTable` in `activitiesTable.jsx`).
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
- Keep API data separate from request flags. Use a boolean `loading` state where the UI needs a progress indicator
  and a string `error` state (`''` when clear); do not introduce a combined `{ status, data, error }` state shape.
  Clear prior errors before a request, and reset `loading` in `finally` when the operation sets it.
- Catch API errors, log the underlying exception with operation context, and put a user-facing message in `error`;
  do not display a raw Axios error object. Return an explicit success/failure result when the caller needs to react
  to the outcome. For overlapping requests, prevent an older response from overwriting newer data, using cancellation
  or a request token as appropriate.

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
- Jest configuration is in `jest.config.js`: tests run in `jsdom`, `src/setupTests.js` registers
  `@testing-library/jest-dom`, and Babel transforms JavaScript/JSX. Test files may use `*.test.js`, `*.test.jsx`,
  `*.spec.js`, `*.spec.jsx`, or live under `__tests__/` within `src/`.
- Use `screen.getByRole` / `getByLabelText` for accessible queries; avoid test IDs and CSS selectors where possible.
- Use `waitFor` for UI state that follows an asynchronous request. Use controlled promises when testing request ordering
  or responses that may arrive out of order.
- For HTTP-backed workflows, mock the Axios/API boundary as needed; unit and component tests should not require a
  running service or database. Use test payloads matching the REST API's JSON contract.
- Presentational components can be tested directly without network mocks. The existing test suite currently covers
  date utilities in `src/lib/dateUtils.test.js`.

## Dependencies
- Frontend dependencies belong in `package.json`; keep Rust-only concerns out of it.
- **SHOULD** check whether React, MUI Joy, or `@tauri-apps/api` already provide the needed functionality before
  adding a package.
- Do not add Tauri plugins on the frontend without the matching Rust plugin registration and capability update.
