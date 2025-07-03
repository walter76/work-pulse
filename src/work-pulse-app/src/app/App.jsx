import { Routes, Route } from 'react-router-dom'
import { Sheet, Typography } from '@mui/joy'

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
    <Navigation />
    <Routes>
      <Route path="/activities" element={<ActivitiesList />} />
      <Route path="/categories" element={<PamCategoriesConfiguration />} />
      <Route path="/" element={<ActivitiesList />} />
    </Routes>
  </Sheet>
)

export default App
