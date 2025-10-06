import { useCallback, useState } from 'react'
import axios from 'axios'

import { API_BASE_URL } from '../config/api'

export const useActivities = () => {
  const [activities, setActivities] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')

  const refreshActivities = useCallback(async (startDate, endDate) => {
    console.log('Refreshing activities...')
    setError('')
    setLoading(true)

    try {
      const response = await axios.get(
        `${API_BASE_URL}/api/v1/activities?start_date=${startDate}&end_date=${endDate}`,
      )

      setActivities(response.data)

      console.log('Activities refreshed successfully!')
    } catch (error) {
      console.error('Error fetching activities:', error)
      setError('Failed to fetch activities. Please try again.')
    } finally {
      setLoading(false)
    }
  }, [])

  return {
    activities,
    loading,
    error,
    setError,
    refreshActivities,
  }
}
