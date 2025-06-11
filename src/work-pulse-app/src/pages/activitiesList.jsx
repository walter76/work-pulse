import { Button, Input, Select, Sheet, Typography } from '@mui/joy'
import { Add } from '@mui/icons-material'

const ActivitiesList = () => (
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5, margin: 5 }}>
      <Typography level="h2">
        Activities
      </Typography>

      <Sheet variant="outlined" sx={{ display: 'flex', gap: 2, padding: 2 }}>
        <Input
          required
          id="activity-date"
          type="date"
          placeholder="Date"
        />
        <Input
          required
          id="start-time"
          type="time"
          placeholder="Start Time"
        />
        <Input
          id="end-time"
          type="time"
          placeholder="End Time"
        />
        <Select
          id="pam-category"
          placeholder="PAM Category">
        </Select>
        <Input
          required
          id="task"
          placeholder="Task"
        />
        <Button startDecorator={<Add/>}>
          Add Activity
        </Button>
      </Sheet>
    </Sheet>
)

export default ActivitiesList
