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
} from '../lib/dateUtils'

import { API_BASE_URL } from '../config/api'

const WeeklyReport = () => {
  const currentWeek = getCurrentWeek()

  const [selectedWeek, setSelectedWeek] = useState(currentWeek)
  const [reportData, setReportData] = useState(null)

  const [error, setError] = useState('')
  const [loading, setLoading] = useState(false)

  const { categories, refreshCategories } = useCategories()

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

  const getCategoryDurationData = () => {
    if (!reportData?.duration_per_category || !categories.length) {
      return []
    }

    return Object.entries(reportData.duration_per_category).map(([categoryId, duration]) => {
      const category = categories.find((cat) => cat.id.toString() === categoryId)

      return {
        categoryId,
        categoryName: category ? category.name : 'Unknown',
        duration,
        formattedDuration: formatDuration(duration),
      }
    })
  }

  const categoryDurationData = getCategoryDurationData()

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
                      <th>Category</th>
                      <th>Duration (HH:MM)</th>
                    </tr>
                  </thead>
                  <tbody>
                    {categoryDurationData
                      .sort((a, b) => a.categoryName.localeCompare(b.categoryName))
                      .map(({ categoryName, formattedDuration }) => (
                        <tr key={categoryName}>
                          <td>{categoryName}</td>
                          <td>{formattedDuration}</td>
                        </tr>
                      ))}
                  </tbody>
                  <tfoot>
                    <tr
                      style={{
                        fontWeight: 'bold',
                        backgroundColor: 'var(--joy-palette-neutral-50)',
                      }}
                    >
                      <td>Total</td>
                      <td>{formatDuration(reportData.total_duration)}</td>
                    </tr>
                  </tfoot>
                </Table>

                {categoryDurationData.length === 0 && (
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
