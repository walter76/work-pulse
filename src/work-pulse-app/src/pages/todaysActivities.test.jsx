import { fireEvent, render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, jest } from '@jest/globals'
import userEvent from '@testing-library/user-event'
import axios from 'axios'

import EditActivity from './editActivity'
import TodaysActivities from './todaysActivities'

const mockCreateActivity = jest.fn()
const mockDeleteActivity = jest.fn()
const mockNavigate = jest.fn()
const mockRefreshActivities = jest.fn()
const mockSetError = jest.fn()
const mockUpdateActivity = jest.fn()
const mockLocation = { state: undefined }
let mockActivities = []

jest.mock('../hooks/useActivities', () => ({
  useActivities: () => ({
    activities: mockActivities,
    loading: false,
    error: '',
    setError: mockSetError,
    refreshActivities: mockRefreshActivities,
    createActivity: mockCreateActivity,
    updateActivity: mockUpdateActivity,
    deleteActivity: mockDeleteActivity,
  }),
}))

jest.mock('../hooks/useCategories', () => ({
  useCategories: () => ({
    categories: [{ id: 'category-1', name: 'Development' }],
    refreshCategories: jest.fn(),
  }),
}))

jest.mock('react-router-dom', () => ({
  useLocation: () => mockLocation,
  useNavigate: () => mockNavigate,
  useParams: () => ({ id: 'one' }),
}))

jest.mock('axios', () => ({ get: jest.fn() }))
jest.mock('../config/api', () => ({ API_BASE_URL: 'http://localhost:8080' }))

const selectedDate = '2026-03-15'

const makeActivity = (id, start_time, end_time) => ({
  id,
  date: selectedDate,
  start_time,
  end_time,
  accounting_category_id: 'category-1',
  task: `Task ${id}`,
  comment: '',
})

const renderPage = (date = selectedDate) => {
  mockLocation.state = date ? { selectedDate: date } : undefined
  return render(<TodaysActivities />)
}

describe('TodaysActivities', () => {
  beforeEach(() => {
    mockActivities = []
    mockCreateActivity.mockReset().mockResolvedValue({ success: true })
    mockDeleteActivity.mockReset().mockResolvedValue(true)
    mockNavigate.mockReset()
    mockRefreshActivities.mockReset().mockResolvedValue(undefined)
    mockSetError.mockReset()
    mockUpdateActivity.mockReset().mockResolvedValue({ success: true })
    axios.get.mockReset().mockResolvedValue({
      data: {
        id: 'one',
        date: selectedDate,
        start_time: '09:00:00',
        end_time: '10:00:00',
        accounting_category_id: 'category-1',
        task: 'Review',
        comment: null,
      },
    })
    mockLocation.state = undefined
  })

  it('defaults to the local current date', async () => {
    renderPage(null)

    const now = new Date()
    const today = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(
      now.getDate(),
    ).padStart(2, '0')}`

    expect(screen.getByLabelText('Selected date')).toHaveValue(today)
    await waitFor(() => {
      expect(mockRefreshActivities).toHaveBeenLastCalledWith(today, today)
    })
  })

  it('loads the selected day and reloads when the date changes', async () => {
    renderPage()

    await waitFor(() => {
      expect(mockRefreshActivities).toHaveBeenLastCalledWith(selectedDate, selectedDate)
    })

    const nextDate = '2026-03-16'
    fireEvent.change(screen.getByLabelText('Selected date'), { target: { value: nextDate } })

    await waitFor(() => {
      expect(mockRefreshActivities).toHaveBeenLastCalledWith(nextDate, nextDate)
    })
  })

  it('shows an empty state and a zero orange total for a day without activities', () => {
    renderPage()

    expect(screen.getByText('No activities found for the selected date.')).toBeInTheDocument()
    expect(screen.getByText('00:00')).toHaveClass('MuiTypography-colorWarning')
  })

  it('shows activity durations and color-codes the daily total', () => {
    mockActivities = [
      makeActivity('one', '08:00:00', '10:30:00'),
      makeActivity('two', '10:30:00', '16:30:00'),
    ]

    const { rerender } = renderPage()

    expect(screen.getByText('02:30')).toBeInTheDocument()
    expect(screen.getByText('06:00')).toBeInTheDocument()
    expect(screen.getByText('08:30')).toHaveClass('MuiTypography-colorSuccess')

    mockActivities = [makeActivity('long', '07:00:00', '18:30:00')]
    rerender(<TodaysActivities />)
    expect(screen.getByText('11:30', { selector: 'p' })).toHaveClass(
      'MuiTypography-colorDanger',
    )
  })

  it('creates an activity for the selected day and refreshes that day', async () => {
    const user = userEvent.setup()
    renderPage()

    fireEvent.change(screen.getByPlaceholderText('Start Time'), { target: { value: '09:00' } })
    fireEvent.change(screen.getByPlaceholderText('End Time'), { target: { value: '10:00' } })
    fireEvent.change(screen.getByPlaceholderText('Task'), { target: { value: 'Review' } })
    await user.click(screen.getByRole('combobox'))
    await user.click(screen.getByRole('option', { name: 'Development' }))
    await user.click(screen.getByRole('button', { name: 'Add Activity' }))

    await waitFor(() => {
      expect(mockCreateActivity).toHaveBeenCalledWith(
        expect.objectContaining({ date: selectedDate, task: 'Review' }),
      )
      expect(mockRefreshActivities).toHaveBeenLastCalledWith(selectedDate, selectedDate)
    })
  })

  it('preserves the selected day when editing and invokes deletion for a row', async () => {
    const user = userEvent.setup()
    mockActivities = [makeActivity('one', '08:00:00', '09:00:00')]
    renderPage()

    await user.click(screen.getByRole('button', { name: 'Edit Activity' }))
    expect(mockNavigate).toHaveBeenCalledWith('/activities/edit/one', {
      state: { selectedDate },
    })

    await user.click(screen.getByRole('button', { name: 'Delete Activity' }))
    expect(mockDeleteActivity).toHaveBeenCalledWith('one')
  })

  it('returns to the selected day after saving an edit', async () => {
    const user = userEvent.setup()
    mockLocation.state = { selectedDate }
    render(<EditActivity />)

    await waitFor(() => expect(screen.getByDisplayValue(selectedDate)).toBeInTheDocument())
    await user.click(screen.getByRole('button', { name: 'Save' }))

    await waitFor(() => {
      expect(mockUpdateActivity).toHaveBeenCalledWith(
        'one',
        expect.objectContaining({ id: 'one', date: selectedDate }),
      )
      expect(mockNavigate).toHaveBeenCalledWith('/activities', { state: { selectedDate } })
    })
  })
})