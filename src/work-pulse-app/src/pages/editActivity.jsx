import { useEffect, useState } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import { Button, Input, Option, Select, Sheet, Typography } from '@mui/joy'
import { Save } from '@mui/icons-material'
import axios from 'axios'

import ErrorMessage from '../components/errorMessage'

import { useActivities } from '../hooks/useActivities'
import { useCategories } from '../hooks/useCategories'
import { API_BASE_URL } from '../config/api'

const EditActivity = () => {
  const { id: activityId } = useParams()

  const { categories, refreshCategories } = useCategories()

  const [activityDate, setActivityDate] = useState('')
  const [startTime, setStartTime] = useState('')
  const [endTime, setEndTime] = useState('')
  const [categoryId, setCategoryId] = useState('')
  const [task, setTask] = useState('')

  const { error, setError, updateActivity } = useActivities()

  useEffect(() => {
    if (activityId) {
      refreshCategories()
      fetchActivity()
    }
  }, [activityId])

  const fetchActivity = async () => {
    console.log(`Fetching activity with ID: ${activityId}`)
    setError('')

    try {
      const response = await axios.get(`${API_BASE_URL}/api/v1/activities/${activityId}`)
      const activityData = response.data

      setActivityDate(activityData.date)
      setStartTime(activityData.start_time)
      setEndTime(activityData.end_time)
      setCategoryId(activityData.accounting_category_id)
      setTask(activityData.task)

      console.log('Activity fetched successfully!')
    } catch (error) {
      console.error('Error fetching activity:', error)
      setError('Failed to fetch activity. Please try again.')
    }
  }

  const navigate = useNavigate()

  const handleUpdateActivity = async () => {
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

    await updateActivity(activityId, {
      id: activityId,
      date: activityDate,
      start_time: startTime,
      end_time: endTime,
      accounting_category_id: categoryId,
      task: task,
    })

    // Reset the input field after updating the activity
    setCategoryId('')
    setActivityDate('')
    setStartTime('')
    setEndTime('')
    setTask('')

    navigate('/activities')
  }

  return (
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5 }}>
      <Typography level="h2">Edit Activity</Typography>

      <ErrorMessage message={error} />

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
          required
          id="start-time"
          type="time"
          placeholder="Start Time"
          value={startTime}
          onChange={(e) => setStartTime(e.target.value)}
          size="sm"
          autoFocus
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
            if (e.key === 'Enter') handleUpdateActivity()
          }}
          size="sm"
          sx={{ minWidth: 400 }}
        />
        <Button startDecorator={<Save />} onClick={handleUpdateActivity} size="sm">
          Save
        </Button>
      </Sheet>
    </Sheet>
  )
}

export default EditActivity
