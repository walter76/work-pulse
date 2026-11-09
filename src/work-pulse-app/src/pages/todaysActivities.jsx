import { useEffect, useRef, useState } from 'react'
import { useLocation, useNavigate } from 'react-router-dom'
import { Button, CircularProgress, Input, Option, Select, Sheet, Typography } from '@mui/joy'
import { Add, Refresh } from '@mui/icons-material'

import ActivitiesTable from '../components/activitiesTable'
import ErrorMessage from '../components/errorMessage'

import { useActivities } from '../hooks/useActivities'
import { useCategories } from '../hooks/useCategories'
import {
  calculateDurationInMinutes,
  formatDateForDisplay,
  formatDuration,
} from '../lib/dateUtils'

/**
 * Lets users review and manage activities for a selected calendar day.
 * @returns {JSX.Element} The selected-day activity workflow.
 */
const TodaysActivities = () => {
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
  const location = useLocation()

  const [categoryId, setCategoryId] = useState('')
  const [selectedDate, setSelectedDate] = useState(() => {
    if (location.state?.selectedDate) return location.state.selectedDate

    const today = new Date()
    return `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(
      today.getDate(),
    ).padStart(2, '0')}`
  })
  const [startTime, setStartTime] = useState('')
  const [endTime, setEndTime] = useState('')
  const [task, setTask] = useState('')
  const [comment, setComment] = useState('')

  const startTimeRef = useRef(null)
  const selectedDateRef = useRef(selectedDate)

  useEffect(() => {
    refreshCategories()
  }, [refreshCategories])

  useEffect(() => {
    refreshActivities(selectedDate, selectedDate)
  }, [refreshActivities, selectedDate])

  const dailyTotalMinutes = activities.reduce(
    (total, activity) =>
      total + calculateDurationInMinutes(activity.start_time, activity.end_time),
    0,
  )
  const dailyTotalHours = dailyTotalMinutes / 60
  const dailyTotalColor = dailyTotalHours < 8 ? 'warning' : dailyTotalHours <= 10 ? 'success' : 'danger'

  const handleDateChanged = (event) => {
    const nextDate = event.target.value
    if (!nextDate) return

    selectedDateRef.current = nextDate
    setSelectedDate(nextDate)
  }

  const handleCreateActivity = async () => {
    if (!selectedDate) {
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

    const activityDate = selectedDate
    const result = await createActivity({
      date: activityDate,
      start_time: startTime,
      end_time: endTime,
      accounting_category_id: categoryId,
      task: task,
      comment: comment || null,
    })

    if (result.success) {
      setStartTime('')
      setEndTime('')
      setTask('')
      setComment('')
      if (selectedDateRef.current === activityDate) {
        refreshActivities(activityDate, activityDate)
      }
    }
  }

  const handleDeleteActivity = async (activityId) => {
    await deleteActivity(activityId)
  }

  const navigate = useNavigate()

  const handleEditActivity = (activity) => {
    console.log(`Editing activity with ID: ${activity.id}`)
    setError('')

    navigate(`/activities/edit/${activity.id}`, { state: { selectedDate } })
  }

  return (
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5 }}>
      <Typography level="h2">Today's Activities</Typography>

      <ErrorMessage message={error} />

      <Sheet variant="outlined" sx={{ display: 'flex', gap: 2, padding: 2 }}>
        <Input
          required
          id="activity-date"
          type="date"
          aria-label="Selected date"
          value={selectedDate}
          onChange={handleDateChanged}
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
        <Input
          id="comment"
          placeholder="Comment"
          value={comment}
          onChange={(e) => setComment(e.target.value)}
          size="sm"
        />
        <Button startDecorator={<Add />} onClick={handleCreateActivity} size="sm">
          Add Activity
        </Button>
      </Sheet>

      <Sheet variant="outlined" sx={{ gap: 2, padding: 2 }}>
        <Typography level="h3">Activities List on {formatDateForDisplay(selectedDate)}</Typography>

        <Sheet
          sx={{ display: 'flex', alignItems: 'center', gap: 2, marginTop: 2, marginBottom: 2 }}
        >
          <Typography>Number of Records: {activities.length}</Typography>
          <Button
            startDecorator={loading ? <CircularProgress size="sm" /> : <Refresh />}
            onClick={() => refreshActivities(selectedDate, selectedDate)}
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
              activities={[...activities].sort((a, b) => b.start_time.localeCompare(a.start_time))}
              categories={categories}
              onEditActivity={handleEditActivity}
              onDeleteActivity={handleDeleteActivity}
              totalDuration={formatDuration(`PT${dailyTotalMinutes}M`)}
              totalDurationColor={dailyTotalColor}
            />

            {activities.length === 0 && (
              <Typography level="body-md" color="neutral" sx={{ padding: 1 }}>
                No activities found for the selected date.
              </Typography>
            )}
          </>
        )}
      </Sheet>
    </Sheet>
  )
}

export default TodaysActivities
