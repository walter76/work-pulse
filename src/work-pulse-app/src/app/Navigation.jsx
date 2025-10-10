import { Link, useLocation } from 'react-router-dom'
import { Button, Divider, Stack } from '@mui/joy'
import { CloudUpload, List, Settings, ViewList } from '@mui/icons-material'

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
        component={Link}
        to="/activities/log"
        variant={location.pathname === '/activities/log' ? 'solid' : 'soft'}
        startDecorator={<ViewList />}
        fullWidth
        sx={{ justifyContent: 'flex-start' }}
      >
        Activity Log
      </Button>

      <Divider />

      <Button
        component={Link}
        to="/daily-report"
        variant={location.pathname === '/daily-report' ? 'solid' : 'soft'}
        startDecorator={<ViewList />}
        fullWidth
        sx={{ justifyContent: 'flex-start' }}
      >
        Daily Report
      </Button>

      <Button
        variant="soft"
        startDecorator={<ViewList />}
        fullWidth
        sx={{ justifyContent: 'flex-start' }}
      >
        Weekly Report
      </Button>

      <Button
        variant="soft"
        startDecorator={<ViewList />}
        fullWidth
        sx={{ justifyContent: 'flex-start' }}
      >
        Yearly Report
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

      <Button
        component={Link}
        to="/import"
        variant={location.pathname === '/import' ? 'solid' : 'soft'}
        startDecorator={<CloudUpload />}
        fullWidth
        sx={{ justifyContent: 'flex-start' }}
      >
        Import Activities
      </Button>
    </Stack>
  )
}

export default Navigation
