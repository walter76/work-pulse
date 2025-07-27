import { Routes, Route } from 'react-router-dom'
import { Box, Sheet, Typography } from '@mui/joy'

import TodaysActivities from "../pages/todaysActivities"
import CategoriesConfiguration from "../pages/categoriesConfiguration"
import EditActivity from "../pages/editActivity"

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
          <Route path="/activities" element={<TodaysActivities />} />
          <Route path="/activities/edit/:id" element={<EditActivity />} />
          <Route path="/categories" element={<CategoriesConfiguration />} />
          <Route path="/" element={<TodaysActivities />} />
        </Routes>
      </Box>
    </Box>
  </Sheet>
)

export default App
