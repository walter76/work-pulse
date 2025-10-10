import { useState } from 'react'
import { Button, Input, Sheet, Table, Typography } from '@mui/joy'
import { Refresh } from '@mui/icons-material'
import axios from 'axios'

import { API_BASE_URL } from '../config/api'

const DailyReport = () => {
  const today = new Date()
  const formattedDate = today.toISOString().split('T')[0] // Format date as YYYY-MM-DD

  const [selectedDate, setSelectedDate] = useState(formattedDate)
  const [reportData, setReportData] = useState(null)

  const [error, setError] = useState('')
  const [loading, setLoading] = useState(false)

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
        <Typography level="h3">Report for {selectedDate}</Typography>

        <Sheet sx={{ display: 'flex', alignItems: 'center', gap: 2, marginBottom: 2 }}>
          <Typography level="body1">
            Total Activities: {reportData?.activities?.length || 0}
          </Typography>
          <Typography fontWeight="bold" color="primary">
            Total Hours: {reportData?.total_duration || 0}
          </Typography>
        </Sheet>

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
                  <th>Duration (hours)</th>
                  <th>Category</th>
                  <th>Activity</th>
                </tr>
              </thead>
              <tbody>
                {reportData?.activities?.map((activity) => {
                  return (
                    <tr key={activity.id}>
                      <td>{activity.start_time}</td>
                      <td>{activity.end_time}</td>
                      <td>{activity.duration}</td>
                      <td>{activity.accounting_category_id}</td>
                      <td>{activity.task}</td>
                    </tr>
                  )
                })}
              </tbody>
            </Table>
          </>
        )}
      </Sheet>
    </Sheet>
  )
}

export default DailyReport
