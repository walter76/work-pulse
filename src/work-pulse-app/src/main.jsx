import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'

import '@fontsource/inter'

import { CssVarsProvider } from '@mui/joy/styles'
import CssBaseline from '@mui/joy/CssBaseline'

import { BrowserRouter as Router } from 'react-router-dom'

import App from './app/App.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <CssVarsProvider>
      <CssBaseline />
      <Router>
        <App />
      </Router>
    </CssVarsProvider>
  </StrictMode>,
)
