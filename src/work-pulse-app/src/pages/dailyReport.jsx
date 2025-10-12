import { useEffect, useState } from 'react'
import { Button, Input, Sheet, Table, Typography } from '@mui/joy'
import { Refresh } from '@mui/icons-material'
import axios from 'axios'

import { useCategories } from '../hooks/useCategories'
import { formatDateForDisplay, formatDuration } from '../lib/dateUtils'

import { API_BASE_URL } from '../config/api'

const DailyReport = () => {
  const today = new Date()
  const formattedDate = today.toISOString().split('T')[0] // Format date as YYYY-MM-DD

  const [selectedDate, setSelectedDate] = useState(formattedDate)
  const [reportData, setReportData] = useState(null)

  const [error, setError] = useState('')
  const [loading, setLoading] = useState(false)

  const { categories, refreshCategories } = useCategories()

  useEffect(() => {
    refreshCategories()
  }, [refreshCategories])

  const handleDateChanged = (e) => {
    setSelectedDate(e.target.value)
  }

  const handleRefresh = async () => {
    console.log(`Refreshing daily report for date: ${selectedDate}`)

    setError('')
    setLoading(true)

    try {
      const response = await axios.get(
        `${API_BASE_URL}/api/v1/daily-report?report_date=${selectedDate}`,
      )

      setReportData(response.data)

      console.log('Daily report refreshed successfully!')
    } catch (error) {
      console.error('Error fetching daily report:', error)
      setError('Failed to fetch daily report. Please try again.')
    } finally {
      setLoading(false)
    }
  }

  return (
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5 }}>
      <Typography level="h2">Daily Report</Typography>

      {error && (
        <Typography level="body2" color="danger" sx={{ padding: 1 }}>
          {error}
        </Typography>
      )}

      <Sheet variant="outlined" sx={{ display: 'flex', gap: 2, padding: 2 }}>
        <Input
          type="date"
          label="Select Date"
          value={selectedDate}
          onChange={handleDateChanged}
          size="sm"
        />
        <Button startDecorator={<Refresh />} onClick={handleRefresh} loading={loading} size="sm">
          Refresh Report
        </Button>
      </Sheet>

      <Sheet variant="outlined" sx={{ gap: 2, padding: 2 }}>
        <Typography level="h3">{formatDateForDisplay(selectedDate)}</Typography>

        {loading ? (
          <Typography level="body-md" sx={{ textAlign: 'center', padding: 3 }}>
            Loading report...
          </Typography>
        ) : (
          <>
            <Table>
              <thead>
                <tr>
                  <th>Start Time</th>
                  <th>End Time</th>
                  <th>Duration (HH:MM)</th>
                  <th>Category</th>
                  <th>Activity</th>
                </tr>
              </thead>
              <tbody>
                {reportData?.activities
                  .sort((a, b) => a.start_time.localeCompare(b.start_time))
                  .map((activity) => {
                    const category = categories.find(
                      (cat) => cat.id === activity.accounting_category_id,
                    )
                    const categoryName = category ? category.name : 'Unknown'

                    return (
                      <tr key={activity.id}>
                        <td>{activity.start_time}</td>
                        <td>{activity.end_time}</td>
                        <td>{formatDuration(activity.duration)}</td>
                        <td>{categoryName}</td>
                        <td>{activity.task}</td>
                      </tr>
                    )
                  })}
              </tbody>
              {reportData?.activities.length > 0 && (
                <tfoot>
                  <tr
                    style={{ fontWeight: 'bold', backgroundColor: 'var(--joy-palette-neutral-50)' }}
                  >
                    <td colSpan={2}>Total:</td>
                    <td>{formatDuration(reportData.total_duration)}</td>
                    <td colSpan={2}></td>
                  </tr>
                </tfoot>
              )}
            </Table>

            {reportData?.activities.length === 0 && (
              <Typography level="body-md" color="neutral" sx={{ textAlign: 'center', padding: 3 }}>
                No activities found for the selected date.
              </Typography>
            )}
          </>
        )}
      </Sheet>
    </Sheet>
  )
}

export default DailyReport
