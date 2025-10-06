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

  const deleteActivity = useCallback(async (activityId) => {
    console.log(`Deleting activity with ID: ${activityId}`)
    setError('')

    try {
      await axios.delete(`${API_BASE_URL}/api/v1/activities/${activityId}`)

      console.log(`Activity with ID ${activityId} deleted successfully!`)

      return true
    } catch (error) {
      console.error(`Error deleting activity with ID ${activityId}:`, error)
      setError('Failed to delete activity. Please try again.')

      return false
    }
  }, [])

  return {
    activities,
    loading,
    error,
    setError,
    refreshActivities,
    deleteActivity,
  }
}
