import { BrowserRouter as Router, Link, Routes, Route, useLocation } from 'react-router-dom'
import { Box, Button, Sheet, Typography } from '@mui/joy'

import ActivitiesList from "../pages/activitiesList"
import PamCategoriesConfiguration from "../pages/pamCategoriesConfiguration"

const Navigation = () => {
  const location = useLocation()

  return (
    <Box sx={{ display: 'flex', gap: 2, marginBottom: 3 }}>
      <Button
        component={Link}
        to="/activities"
        variant={location.pathname === '/activities' ? 'solid' : 'soft'}
      >
        Activities Tracker
      </Button>
      <Button
        component={Link}
        to="/categories"
        variant={location.pathname === '/categories' ? 'solid' : 'soft'}
      >
        Categories Configuration
      </Button>
    </Box>
  )
}

const AppContent = () => (
  <Sheet sx={{ minHeight: '100vh', padding: 2 }}>
    <Sheet
      variant="outlined"
      sx={{
        padding: 3,
        marginBottom: 2,
        backgroundColor: 'primary.50'
      }}
    >
      <Typography level="h1" sx={{ textAlign: 'center' }}>
        Work Pulse
      </Typography>
    </Sheet>
    <Navigation />
    <Routes>
      <Route path="/activities" element={<ActivitiesList />} />
      <Route path="/categories" element={<PamCategoriesConfiguration />} />
      <Route path="/" element={<ActivitiesList />} />
    </Routes>
  </Sheet>
)

const App = () => {
  return (
    <Router>
      <AppContent />
    </Router>
  )
}

export default App
