import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'

import '@fontsource/inter'

import { CssVarsProvider } from '@mui/joy/styles'
import CssBaseline from '@mui/joy/CssBaseline'

import App from './app/App.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <CssVarsProvider>
      <CssBaseline />
      <App />
    </CssVarsProvider>
  </StrictMode>,
)
