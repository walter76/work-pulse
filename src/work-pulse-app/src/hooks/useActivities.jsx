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

  const createActivity = useCallback(async (activityData) => {
    console.log(
      `Creating activity: ${JSON.stringify(activityData)}`,
    )

    setError('')

    try {
      const response = await axios.post(`${API_BASE_URL}/api/v1/activities`, activityData)

      setActivities(prev => [...prev, response.data])

      console.log(`Activity created successfully!`)

      return { success: true, data: response.data }
    } catch (error) {
      console.error('Error creating activity:', error)
      setError('Failed to create activity. Please try again.')

      return { success: false, error: error.message }
    }
  }, [])

  const updateActivity = useCallback(async (activityId, activityData) => {
    console.log(
      `Updating activity with ID ${activityId}: ${JSON.stringify(activityData)}`,
    )

    setError('')

    try {
      const response = await axios.put(`${API_BASE_URL}/api/v1/activities`, activityData)

      setActivities(prev =>
        prev.map(act => (act.id === activityId ? response.data : act)),
      )

      console.log(`Activity with ID ${activityId} updated successfully!`)

      return { success: true, data: response.data }
    } catch (error) {
      console.error(`Error updating activity with ID ${activityId}:`, error)
      setError('Failed to update activity. Please try again.')

      return { success: false, error: error.message }
    }
  }, [])

  const deleteActivity = useCallback(async (activityId) => {
    console.log(`Deleting activity with ID: ${activityId}`)

    setError('')

    const originalActivities = [...activities]
    setActivities(prev => prev.filter(act => act.id !== activityId))

    try {
      await axios.delete(`${API_BASE_URL}/api/v1/activities/${activityId}`)

      console.log(`Activity with ID ${activityId} deleted successfully!`)

      return true
    } catch (error) {
      setActivities(originalActivities) // Revert optimistic update

      console.error(`Error deleting activity with ID ${activityId}:`, error)
      setError('Failed to delete activity. Please try again.')

      return false
    }
  }, [activities])

  return {
    activities,
    loading,
    error,
    setError,
    refreshActivities,
    createActivity,
    updateActivity,
    deleteActivity,
  }
}
