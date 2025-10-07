import { useEffect, useRef, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Button, CircularProgress, Input, Option, Select, Sheet, Typography } from '@mui/joy'
import { Add, Refresh } from '@mui/icons-material'

import { useActivities } from '../hooks/useActivities'
import ActivitiesTable from '../components/activitiesTable'
import { useCategories } from '../hooks/useCategories'

const TodaysActivities = () => {
  const today = new Date()
  const formattedDate = today.toISOString().split('T')[0] // Format date as YYYY-MM-DD

  const {
    activities,
    loading,
    error,
    setError,
    refreshActivities,
    createActivity,
    deleteActivity,
  } = useActivities()

  const { categories, refreshCategories } = useCategories()

  const [categoryId, setCategoryId] = useState('')
  const [activityDate, setActivityDate] = useState(formattedDate)
  const [startTime, setStartTime] = useState('')
  const [endTime, setEndTime] = useState('')
  const [task, setTask] = useState('')

  const startTimeRef = useRef(null)

  useEffect(() => {
    refreshCategories()
    refreshActivities(formattedDate, formattedDate)
  }, [refreshActivities, refreshCategories])

  const handleCreateActivity = async () => {
    if (!activityDate) {
      setError('Please enter a valid date for the activity.')
      return
    }

    if (!startTime) {
      setError('Please enter a start time for the activity.')
      return
    }

    if (endTime && new Date(`1970-01-01T${endTime}`) <= new Date(`1970-01-01T${startTime}`)) {
      setError('Please enter a valid end time. End time must be after start time.')
      return
    }

    if (!categoryId) {
      setError('Please select a category for the activity.')
      return
    }

    if (!task) {
      setError('Please enter a task for the activity.')
      return
    }

    await createActivity({
      date: activityDate,
      start_time: startTime,
      end_time: endTime,
      pam_category_id: categoryId,
      task: task,
    })
  }

  const handleDeleteActivity = async (activityId) => {
    const success = await deleteActivity(activityId)

    if (success) {
      // Refresh the activities list after deletion
      refreshActivities(formattedDate, formattedDate)
    }
  }

  const navigate = useNavigate()

  const handleEditActivity = (activity) => {
    console.log(`Editing activity with ID: ${activity.id}`)
    setError('')

    navigate(`/activities/edit/${activity.id}`)
  }

  return (
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5 }}>
      <Typography level="h2">Today's Activities</Typography>

      {error && (
        <Typography level="body-md" color="danger" sx={{ padding: 1 }}>
          {error}
        </Typography>
      )}

      <Sheet variant="outlined" sx={{ display: 'flex', gap: 2, padding: 2 }}>
        <Input
          required
          id="activity-date"
          type="date"
          placeholder="Date"
          value={activityDate}
          onChange={(e) => setActivityDate(e.target.value)}
          size="sm"
        />
        <Input
          ref={startTimeRef}
          required
          id="start-time"
          type="time"
          placeholder="Start Time"
          value={startTime}
          onChange={(e) => setStartTime(e.target.value)}
          size="sm"
          autoFocus
          slotProps={{
            input: {
              ref: startTimeRef,
            },
          }}
        />
        <Input
          id="end-time"
          type="time"
          placeholder="End Time"
          value={endTime}
          onChange={(e) => setEndTime(e.target.value)}
          size="sm"
        />
        <Select
          id="category"
          placeholder="Category"
          value={categoryId}
          onChange={(_e, newValue) => setCategoryId(newValue)}
          size="sm"
        >
          {categories.map((category) => (
            <Option key={category.id} value={category.id}>
              {category.name}
            </Option>
          ))}
        </Select>
        <Input
          required
          id="task"
          placeholder="Task"
          value={task}
          onChange={(e) => setTask(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') handleCreateActivity()
          }}
          size="sm"
          sx={{ minWidth: 400 }}
        />
        <Button startDecorator={<Add />} onClick={handleCreateActivity} size="sm">
          Add Activity
        </Button>
      </Sheet>

      <Sheet variant="outlined" sx={{ gap: 2, padding: 2 }}>
        <Typography level="h3">Activities List on {formattedDate}</Typography>

        <Sheet
          sx={{ display: 'flex', alignItems: 'center', gap: 2, marginTop: 2, marginBottom: 2 }}
        >
          <Typography>Number of Records: {activities.length}</Typography>
          <Button
            startDecorator={loading ? <CircularProgress size="sm" /> : <Refresh />}
            onClick={() => refreshActivities(formattedDate, formattedDate)}
            size="sm"
            loading={loading}
          >
            Refresh Activities
          </Button>
        </Sheet>

        {loading ? (
          <Sheet
            sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', padding: 4 }}
          >
            <CircularProgress size="lg" />
            <Typography level="body-md" sx={{ marginLeft: 2 }}>
              Loading activities...
            </Typography>
          </Sheet>
        ) : (
          <>
            <ActivitiesTable
              activities={activities.sort((a, b) => b.start_time.localeCompare(a.start_time))}
              categories={categories}
              onEditActivity={handleEditActivity}
              onDeleteActivity={handleDeleteActivity}
            />

            {activities.length === 0 && (
              <Typography level="body-md" color="neutral" sx={{ padding: 1 }}>
                No activities found for today.
              </Typography>
            )}
          </>
        )}
      </Sheet>
    </Sheet>
  )
}

export default TodaysActivities
