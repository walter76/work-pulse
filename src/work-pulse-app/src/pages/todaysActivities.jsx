import { useEffect, useRef, useState } from 'react'
import { Button, IconButton, Input, Option, Select, Sheet, Table, Typography } from '@mui/joy'
import { Add, Delete, Refresh } from '@mui/icons-material'
import axios from 'axios'

const TodaysActivities = () => {
  const today = new Date()
  const formattedDate = today.toISOString().split('T')[0] // Format date as YYYY-MM-DD

  const [activities, setActivities] = useState([])
  const [categories, setCategories] = useState([])
  const [categoryId, setCategoryId] = useState('')
  const [activityDate, setActivityDate] = useState(formattedDate)
  const [startTime, setStartTime] = useState('')
  const [endTime, setEndTime] = useState('')
  const [task, setTask] = useState('')
  const [error, setError] = useState('')

  const startTimeRef = useRef(null)
  
  useEffect(() => {
    refreshCategories()
    refreshActivities()
  }, [])

  const refreshActivities = async () => {
    console.log('Refreshing activities...')
    setError('')

    try {
      const response = await axios.get(`http://localhost:8080/api/v1/activities?start_date=${formattedDate}&end_date=${formattedDate}`)

      setActivities(response.data)

      // focus back to start time input
      startTimeRef.current?.focus()

      console.log('Activities refreshed successfully!')
    } catch (error) {
      console.error('Error fetching activities:', error)
      setError('Failed to fetch activities. Please try again.')
    }
  }

  const refreshCategories = async () => {
    console.log('Refreshing categories...')
    setError('')

    try {
      const response = await axios.get('http://localhost:8080/api/v1/pam-categories')

      setCategories(response.data)

      console.log('Categories refreshed successfully!')
    } catch (error) {
      console.error('Error fetching categories:', error)
      setError('Failed to fetch categories. Please try again.')
    }
  }

  const createActivity = async () => {
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

    console.log(`Creating activity for date: ${activityDate}, start time: ${startTime}, end time: ${endTime}, category ID: ${categoryId}, task: ${task}`)
    setError('')

    try {
      await axios.post('http://localhost:8080/api/v1/activities', {
        date: activityDate,
        start_time: startTime,
        end_time: endTime,
        pam_category_id: categoryId,
        task: task
      })

      // Reset the input field after creating the category
      setCategoryId('')
      setActivityDate(formattedDate)
      setStartTime('')
      setEndTime('')
      setTask('')

      // Refresh the activities list after creating the activity
      refreshActivities()

      // focus back to start time input
      startTimeRef.current?.focus()

      console.log(`Activity created successfully!`)
    } catch (error) {
      console.error('Error creating activity:', error)
      setError('Failed to create activity. Please try again.')
    }    
  }

  const deleteActivity = async (activityId) => {
    console.log(`Deleting activity with ID: ${activityId}`)
    setError('')

    try {
      await axios.delete(`http://localhost:8080/api/v1/activities/${activityId}`)

      // Refresh the activities list after deletion
      refreshActivities()

      console.log(`Activity with ID ${activityId} deleted successfully!`)
    } catch (error) {
      console.error(`Error deleting activity with ID ${activityId}:`, error)
      setError('Failed to delete activity. Please try again.')
    }
  }

  return (
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5 }}>
      <Typography level="h2">
        Today's Activities
      </Typography>

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
            }
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
          onKeyDown={ e => {
            if (e.key === 'Enter') createActivity()
          }}
          size="sm"
          sx={{ minWidth: 400 }}
        />
        <Button startDecorator={<Add/>} onClick={createActivity} size="sm">
          Add Activity
        </Button>
      </Sheet>

      <Sheet variant="outlined" sx={{ gap: 2, padding: 2 }}>
        <Typography level="h3">
          Activities List on {formattedDate}
        </Typography>

        <Sheet sx={{ display: 'flex', alignItems: 'center', gap: 2, marginTop: 2, marginBottom: 2 }}>
          <Typography>Number of Records: {activities.length}</Typography>
          <Button startDecorator={<Refresh />} onClick={refreshActivities} size="sm">
            Refresh Activities
          </Button>
        </Sheet>

        <Table>
          <thead>
            <tr>
              <th>Date</th>
              <th>Check-In</th>
              <th>Check-Out</th>
              <th>Category</th>
              <th>Task</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {activities
              .sort((a, b) => {
                // first sort by date (descending)
                const dateComparison = new Date(a.date) - new Date(b.date)
                if (dateComparison !== 0) return dateComparison

                // if dates are equal, sort by start time (descending)
                return a.start_time.localeCompare(b.start_time)
              })
              .map((activity) => {
                const category = categories.find((cat) => cat.id === activity.pam_category_id)
                const categoryName = category ? category.name : 'Unknown'

                return (
                  <tr key={activity.id}>
                    <td>{activity.date}</td>
                    <td>{activity.start_time}</td>
                    <td>{activity.end_time}</td>
                    <td>{categoryName}</td>
                    <td>{activity.task}</td>
                    <td>
                      <IconButton 
                        aria-label="Delete Activity"
                        color="danger"
                        variant="soft"
                        onClick={() => deleteActivity(activity.id)}
                        size="sm"
                      >
                        <Delete />
                      </IconButton>
                    </td>
                  </tr>
                )
            })}
          </tbody>
        </Table>

      </Sheet>

    </Sheet>
  )
}

export default TodaysActivities
