import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Button, Divider, IconButton, Input, Sheet, Table, Typography } from '@mui/joy'
import { Delete, Edit, Refresh } from '@mui/icons-material'
import axios from 'axios'

// Helper function to get week number from a date
const getWeekNumber = (date) => {
  const d = new Date(date)
  d.setHours(0, 0, 0, 0)
  d.setDate(d.getDate() + 3 - ((d.getDay() + 6) % 7))
  const week1 = new Date(d.getFullYear(), 0, 4)
  return 1 + Math.round(((d - week1) / 86400000 - 3 + ((week1.getDay() + 6) % 7)) / 7)
}

// Helper function to get the start and end dates of the week for a given date (Monday to Sunday)
const getWeekRange = (date) => {
  const d = new Date(date)
  const day = d.getDay()
  const diffToMonday = d.getDate() - day + (day === 0 ? -6 : 1)
  const monday = new Date(d.setDate(diffToMonday))
  const sunday = new Date(monday)
  sunday.setDate(monday.getDate() + 6)

  return {
    start: monday.toISOString().split('T')[0],
    end: sunday.toISOString().split('T')[0],
  }
}

// Function to group activities by week
const groupActivitiesByWeek = (activities) => {
  const grouped = {}

  activities.forEach((activity) => {
    const weekNumber = getWeekNumber(activity.date)
    const year = new Date(activity.date).getFullYear()
    const weekKey = `${year}-W${weekNumber.toString().padStart(2, '0')}`
    const weekRange = getWeekRange(activity.date)

    if (!grouped[weekKey]) {
      grouped[weekKey] = {
        weekNumber,
        year,
        weekRange,
        activities: [],
      }
    }

    grouped[weekKey].activities.push(activity)
  })

  return grouped
}

// Function to get the first and last day of the current month
const getCurrentMonthRange = () => {
  const now = new Date()
  const firstDay = new Date(now.getFullYear(), now.getMonth(), 1)
  const lastDay = new Date(now.getFullYear(), now.getMonth() + 1, 0)

  return {
    start: firstDay.toISOString().split('T')[0],
    end: lastDay.toISOString().split('T')[0],
  }
}

const ActivityLog = () => {
  const currentMonthRange = getCurrentMonthRange()

  const [fromDate, setFromDate] = useState(currentMonthRange.start)
  const [toDate, setToDate] = useState(currentMonthRange.end)

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

  const groupedActivities = groupActivitiesByWeek(activities)
  const sortedWeeks = Object.keys(groupedActivities).sort()

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
          <Typography>Weeks: {sortedWeeks.length}</Typography>
        </Sheet>

        {sortedWeeks.map((weekKey, weekIndex) => {
          const weekData = groupedActivities[weekKey]
          const sortedActivities = weekData.activities.sort((a, b) => {
            const dateComparison = new Date(a.date) - new Date(b.date)
            if (dateComparison !== 0) return dateComparison

            return a.start_time.localeCompare(b.start_time)
          })

          return (
            <Sheet key={weekKey} sx={{ marginBottom: 3 }}>
              <Typography level="h4" sx={{ marginBottom: 2, color: 'primary.500' }}>
                Week {weekData.weekNumber}, {weekData.year}
                <Typography level="body-sm" sx={{ marginLeft: 1 }}>
                  ({weekData.weekRange.start} to {weekData.weekRange.end}) -{' '}
                  {weekData.activities.length} activities
                </Typography>
              </Typography>

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
                  {sortedActivities.map((activity) => {
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

              {weekIndex < sortedWeeks.length - 1 && <Divider sx={{ marginTop: 2 }} />}
            </Sheet>
          )
        })}

        {activities.length === 0 && (
          <Typography level="body-md" sx={{ textAlign: 'center', padding: 3 }}>
            No activities found for the selected date range.
          </Typography>
        )}
      </Sheet>
    </Sheet>
  )
}

export default ActivityLog
