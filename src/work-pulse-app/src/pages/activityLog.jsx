import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Button, CircularProgress, Divider, Input, Sheet, Typography } from '@mui/joy'
import { Refresh } from '@mui/icons-material'

import { useActivities } from '../hooks/useActivities'
import { useCategories } from '../hooks/useCategories'
import ActivitiesTable from '../components/activitiesTable'

import { getCurrentMonthRange, groupActivitiesByWeek } from '../lib/dateUtils'

const ActivityLog = () => {
  const currentMonthRange = getCurrentMonthRange()

  const [fromDate, setFromDate] = useState(currentMonthRange.start)
  const [toDate, setToDate] = useState(currentMonthRange.end)

  const { activities, loading, error, setError, refreshActivities, deleteActivity } =
    useActivities()
  const { categories, refreshCategories } = useCategories()

  useEffect(() => {
    refreshCategories()
    refreshActivities(fromDate, toDate)
  }, [refreshActivities, refreshCategories])

  const handleDeleteActivity = async (activityId) => {
    const success = await deleteActivity(activityId)

    if (success) {
      // Refresh the activities list after deletion
      refreshActivities(fromDate, toDate)
    }
  }

  const navigate = useNavigate()

  const handleEditActivity = (activity) => {
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
        <Button
          startDecorator={loading ? <CircularProgress size="sm" /> : <Refresh />}
          onClick={() => refreshActivities(fromDate, toDate)}
          size="sm"
          loading={loading}
        >
          Refresh Activities
        </Button>
      </Sheet>

      {loading ? (
        <Sheet sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', padding: 4 }}>
          <CircularProgress size="lg" />
          <Typography level="body-md" sx={{ marginLeft: 2 }}>
            Loading activities...
          </Typography>
        </Sheet>
      ) : (
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
                  onEditActivity={handleEditActivity}
                  onDeleteActivity={handleDeleteActivity}
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
      )}
    </Sheet>
  )
}

export default ActivityLog
