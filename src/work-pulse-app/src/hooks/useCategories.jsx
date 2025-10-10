import { useCallback, useState } from 'react'
import axios from 'axios'

import { API_BASE_URL } from '../config/api'

export const useCategories = () => {
  const [categories, setCategories] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')

  const refreshCategories = useCallback(async () => {
    console.log('Refreshing categories...')

    setError('')
    setLoading(true)

    try {
      const response = await axios.get(`${API_BASE_URL}/api/v1/accounting-categories`)

      setCategories(response.data)

      console.log('Categories refreshed successfully!')
    } catch (error) {
      console.error('Error fetching categories:', error)
      setError('Failed to fetch categories. Please try again.')
    } finally {
        setLoading(false)
    }
  }, [])

  return {
    categories,
    loading,
    error,
    refreshCategories,
  }
}
