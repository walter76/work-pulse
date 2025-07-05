import { Routes, Route } from 'react-router-dom'
import { Box, Sheet, Typography } from '@mui/joy'

import ActivitiesList from "../pages/activitiesList"
import PamCategoriesConfiguration from "../pages/pamCategoriesConfiguration"

import Navigation from './Navigation'

const App = () => (
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

    <Box sx={{ display: 'flex', minHeight: 'calc(100vh - 200px)' }}>
      <Sheet
        variant="outlined"
        sx={{
          width: 300,
          padding: 2,
          borderRight: '1px solid',
          borderColor: 'divider',
        }}
      >
        <Navigation />
      </Sheet>

      <Box sx={{ flex: 1, padding: 2 }}>
        <Routes>
          <Route path="/activities" element={<ActivitiesList />} />
          <Route path="/categories" element={<PamCategoriesConfiguration />} />
          <Route path="/" element={<ActivitiesList />} />
        </Routes>
      </Box>
    </Box>
  </Sheet>
)

export default App
