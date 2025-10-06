import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Button, Divider, Input, Sheet, Typography } from '@mui/joy'
import { Refresh } from '@mui/icons-material'
import axios from 'axios'

import { useActivities } from '../hooks/useActivities'
import ActivitiesTable from '../components/activitiesTable'

import { API_BASE_URL } from '../config/api'
import { getCurrentMonthRange, groupActivitiesByWeek } from '../lib/dateUtils'

const ActivityLog = () => {
  const currentMonthRange = getCurrentMonthRange()

  const [fromDate, setFromDate] = useState(currentMonthRange.start)
  const [toDate, setToDate] = useState(currentMonthRange.end)

  const { activities, loading, error, setError, refreshActivities } = useActivities()
  const [categories, setCategories] = useState([])

  useEffect(() => {
    refreshCategories()
    refreshActivities(fromDate, toDate)
  }, [refreshActivities])

  const refreshCategories = async () => {
    console.log('Refreshing categories...')
    setError('')

    try {
      const response = await axios.get(`${API_BASE_URL}/api/v1/pam-categories`)

      setCategories(response.data)

      console.log('Categories refreshed successfully!')
    } catch (error) {
      console.error('Error fetching categories:', error)
      setError('Failed to fetch categories. Please try again.')
    }
  }

  const deleteActivity = async (activityId) => {
    console.log(`Deleting activity with ID: ${activityId}`)
    setError('')

    try {
      await axios.delete(`${API_BASE_URL}/api/v1/activities/${activityId}`)

      // Refresh the activities list after deletion
      refreshActivities(fromDate, toDate)

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
        <Button startDecorator={<Refresh />} onClick={() => refreshActivities(fromDate, toDate)} size="sm">
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

              <ActivitiesTable
                activities={sortedActivities}
                categories={categories}
                onEditActivity={editActivity}
                onDeleteActivity={deleteActivity}
              />

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
