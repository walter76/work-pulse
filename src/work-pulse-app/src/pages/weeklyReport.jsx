import { useEffect, useState } from 'react'
import { Button, Input, Sheet, Table, Typography } from '@mui/joy'
import { Refresh } from '@mui/icons-material'
import axios from 'axios'

import ErrorMessage from '../components/errorMessage'

import { useCategories } from '../hooks/useCategories'
import {
  getCurrentWeek,
  getWeekStartDate,
  formatDuration,
  formatWeekForDisplay,
  durationToMinutes,
} from '../lib/dateUtils'

import { API_BASE_URL } from '../config/api'

const WeeklyReport = () => {
  const currentWeek = getCurrentWeek()

  const [selectedWeek, setSelectedWeek] = useState(currentWeek)
  const [reportData, setReportData] = useState(null)

  const [error, setError] = useState('')
  const [loading, setLoading] = useState(false)

  const { categories, refreshCategories } = useCategories()

  const getTotalDurationColor = (totalDuration) => {
    const totalMinutes = durationToMinutes(formatDuration(totalDuration))
    const totalHours = totalMinutes / 60

    if (totalHours < 40) {
      return 'warning' // Orange - below 40 hours
    } else if (totalHours >= 40 && totalHours <= 50) {
      return 'success' // Green - between 40 and 50 hours
    } else {
      return 'danger' // Red - above 50 hours
    }
  }

  useEffect(() => {
    refreshCategories()
  }, [refreshCategories])

  const handleWeekChanged = (e) => {
    setSelectedWeek(e.target.value)
  }

  const handleRefresh = async () => {
    console.log(`Refreshing weekly report for week: ${selectedWeek}`)

    setError('')
    setLoading(true)

    // calculate the start date of the week (Monday)
    const startDate = getWeekStartDate(selectedWeek)

    try {
      const response = await axios.get(
        `${API_BASE_URL}/api/v1/weekly-report?week_start_date=${
          startDate.toISOString().split('T')[0]
        }`,
      )

      setReportData(response.data)

      console.log('Weekly report refreshed successfully!')
    } catch (error) {
      console.error('Error fetching weekly report:', error)
      setError('Failed to fetch weekly report. Please try again.')
    } finally {
      setLoading(false)
    }
  }

  // Helper function to format date for display (e.g., "Mon 2025-10-14")
  const formatDateForDisplay = (dateString) => {
    const date = new Date(dateString)
    return date.toLocaleDateString(undefined, {
      weekday: 'short',
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
    })
  }

  // Get all unique categories that appear in the weekly report
  const getReportCategories = () => {
    if (!reportData?.daily_durations_per_category) return []

    const categoryIds = new Set()

    Object.values(reportData.daily_durations_per_category).forEach((dailyData) => {
      Object.keys(dailyData).forEach((categoryId) => categoryIds.add(categoryId))
    })

    return Array.from(categoryIds)
      .map((categoryId) => {
        const category = categories.find((cat) => cat.id.toString() === categoryId)

        return {
          id: categoryId,
          name: category ? category.name : 'Unknown',
        }
      })
      .sort((a, b) => a.name.localeCompare(b.name))
  }

  // Calculate total duration per category across all days
  const getCategoryTotals = () => {
    if (!reportData?.duration_per_category) return {}

    const totals = {}

    Object.entries(reportData.duration_per_category).forEach(([categoryId, duration]) => {
      totals[categoryId] = formatDuration(duration)
    })

    return totals
  }

  const reportCategories = getReportCategories()
  const categoryTotals = getCategoryTotals()
  const totalDurationColor = reportData
    ? getTotalDurationColor(reportData.total_duration)
    : 'neutral'

  return (
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5 }}>
      <Typography level="h2">Weekly Report</Typography>

      <ErrorMessage message={error} />

      <Sheet variant="outlined" sx={{ display: 'flex', gap: 2, padding: 2 }}>
        <Input
          type="week"
          label="Select Week"
          value={selectedWeek}
          onChange={handleWeekChanged}
          size="sm"
        />
        <Button startDecorator={<Refresh />} onClick={handleRefresh} loading={loading} size="sm">
          Refresh Report
        </Button>
      </Sheet>

      <Sheet variant="outlined" sx={{ gap: 2, padding: 2 }}>
        <Typography level="h3">{formatWeekForDisplay(selectedWeek)}</Typography>

        {loading ? (
          <Typography level="body-md" sx={{ textAlign: 'center', padding: 3 }}>
            Loading report...
          </Typography>
        ) : (
          <>
            {reportData ? (
              <>
                <Table>
                  <thead>
                    <tr>
                      <th>Day</th>
                      {reportCategories.map((category) => (
                        <th key={category.id}>{category.name} (HH:MM)</th>
                      ))}
                      <th>Daily Total (HH:MM)</th>
                    </tr>
                  </thead>
                  <tbody>
                    {Object.entries(reportData.daily_durations_per_category || {})
                      .sort(([dateA], [dateB]) => dateA.localeCompare(dateB))
                      .map(([date, dailyData]) => {
                        // Calculate daily total
                        const dailyTotal = Object.values(dailyData).reduce((total, duration) => {
                          const minutes = durationToMinutes(formatDuration(duration))

                          return total + minutes
                        }, 0)
                        const dailyTotalFormatted = formatDuration(`PT${dailyTotal}M`)

                        return (
                          <tr key={date}>
                            <td>{formatDateForDisplay(date)}</td>
                            {reportCategories.map((category) => (
                              <td key={category.id}>
                                {formatDuration(dailyData[category.id] || 0)}
                              </td>
                            ))}
                            <td>{dailyTotalFormatted}</td>
                          </tr>
                        )
                      })}
                  </tbody>
                  <tfoot>
                    <tr
                      style={{
                        fontWeight: 'bold',
                        backgroundColor: 'var(--joy-palette-neutral-50)',
                      }}
                    >
                      <td>Total:</td>
                      {reportCategories.map((category) => (
                        <td key={category.id}>{categoryTotals[category.id] || '00:00'}</td>
                      ))}
                      <td>
                        <Typography color={totalDurationColor} fontWeight="bold">
                          {formatDuration(reportData.total_duration)}
                        </Typography>
                      </td>
                    </tr>
                  </tfoot>
                </Table>

                {Object.keys(reportData.daily_durations_per_category || {}).length === 0 && (
                  <Typography
                    level="body-md"
                    color="neutral"
                    sx={{ textAlign: 'center', padding: 3 }}
                  >
                    No category data available for this week.
                  </Typography>
                )}
              </>
            ) : (
              <Typography level="body-md" color="neutral" sx={{ textAlign: 'center', padding: 3 }}>
                No report data available. Please select a week and refresh.
              </Typography>
            )}
          </>
        )}
      </Sheet>
    </Sheet>
  )
}

export default WeeklyReport
