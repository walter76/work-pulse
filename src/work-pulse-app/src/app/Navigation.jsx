import { Link, useLocation } from 'react-router-dom'
import { Box, Button } from '@mui/joy'

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

export default Navigation
