import { Link, useLocation } from 'react-router-dom'
import { Button, Divider, Stack } from '@mui/joy'
import { List, Settings, ViewList } from '@mui/icons-material'

const Navigation = () => {
  const location = useLocation()

  return (
    <Stack spacing={2}>
      <Button
        component={Link}
        to="/activities"
        variant={location.pathname === '/activities' ? 'solid' : 'soft'}
        startDecorator={<List />}
        fullWidth
        sx={{ justifyContent: 'flex-start' }}
      >
        Today's Activities
      </Button>

      <Divider />

      <Button
        variant="soft"
        startDecorator={<ViewList />}
        fullWidth
        sx={{ justifyContent: 'flex-start' }}
      >
        Daily Activity Log
      </Button>

      <Button
        variant="soft"
        startDecorator={<ViewList />}
        fullWidth
        sx={{ justifyContent: 'flex-start' }}
      >
        Weekly Activity Log
      </Button>

      <Button
        variant="soft"
        startDecorator={<ViewList />}
        fullWidth
        sx={{ justifyContent: 'flex-start' }}
      >
        Yearly Activity Log
      </Button>

      <Divider />

      <Button
        component={Link}
        to="/categories"
        variant={location.pathname === '/categories' ? 'solid' : 'soft'}
        startDecorator={<Settings />}
        fullWidth
        sx={{ justifyContent: 'flex-start' }}
      >
        Categories Configuration
      </Button>
    </Stack>
  )
}

export default Navigation
