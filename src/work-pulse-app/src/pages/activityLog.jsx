import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Button, IconButton, Input, Sheet, Table, Typography } from '@mui/joy'
import { Delete, Edit, Refresh } from '@mui/icons-material'
import axios from 'axios'

const ActivityLog = () => {
  const [fromDate, setFromDate] = useState('2025-01-01')
  const [toDate, setToDate] = useState('2025-10-31')

  const [activities, setActivities] = useState([])
  const [categories, setCategories] = useState([])

  const [error, setError] = useState('')

  useEffect(() => {
    refreshCategories()
    refreshActivities()
  }, [])

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

  const refreshActivities = async () => {
    console.log('Refreshing activities...')
    setError('')

    try {
      const response = await axios.get(
        `http://localhost:8080/api/v1/activities?start_date=${fromDate}&end_date=${toDate}`,
      )

      setActivities(response.data)

      console.log('Activities refreshed successfully!')
    } catch (error) {
      console.error('Error fetching activities:', error)
      setError('Failed to fetch activities. Please try again.')
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

  const navigate = useNavigate()

  const editActivity = (activity) => {
    console.log(`Editing activity with ID: ${activity.id}`)
    setError('')

    navigate(`/activities/edit/${activity.id}`)
  }

  return (
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5 }}>
      <Typography level="h2">Activities Log</Typography>

      {error && (
        <Typography level="body-md" color="danger" sx={{ padding: 1 }}>
          {error}
        </Typography>
      )}

      <Sheet variant="outlined" sx={{ display: 'flex', gap: 2, padding: 2 }}>
        <Input
          type="date"
          placeholder="From Date"
          value={fromDate}
          onChange={(e) => setFromDate(e.target.value)}
          size="sm"
        />
        <Input
          type="date"
          placeholder="To Date"
          value={toDate}
          onChange={(e) => setToDate(e.target.value)}
          size="sm"
        />
        <Button startDecorator={<Refresh />} onClick={refreshActivities} size="sm">
          Refresh Activities
        </Button>
      </Sheet>

      <Sheet variant="outlined" sx={{ gap: 2, padding: 2 }}>
        <Sheet
          sx={{ display: 'flex', alignItems: 'center', gap: 2, marginTop: 2, marginBottom: 2 }}
        >
          <Typography>Number of Records: {activities.length}</Typography>
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
                        aria-label="Edit Activity"
                        color="primary"
                        variant="soft"
                        onClick={() => editActivity(activity)}
                        size="sm"
                      >
                        <Edit />
                      </IconButton>
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

export default ActivityLog
