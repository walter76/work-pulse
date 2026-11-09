import { act, renderHook } from '@testing-library/react'
import { beforeEach, describe, expect, it, jest } from '@jest/globals'
import axios from 'axios'

import { useActivities } from './useActivities'

jest.mock('axios', () => ({ get: jest.fn() }))
jest.mock('../config/api', () => ({ API_BASE_URL: 'http://localhost:8080' }))

describe('useActivities', () => {
  beforeEach(() => {
    axios.get.mockReset()
  })

  it('keeps the newest range result when requests resolve out of order', async () => {
    let resolveOlder
    let resolveNewer
    axios.get
      .mockReturnValueOnce(new Promise((resolve) => (resolveOlder = resolve)))
      .mockReturnValueOnce(new Promise((resolve) => (resolveNewer = resolve)))

    const { result } = renderHook(() => useActivities())
    let olderRequest
    let newerRequest

    await act(async () => {
      olderRequest = result.current.refreshActivities('2026-03-14', '2026-03-14')
      newerRequest = result.current.refreshActivities('2026-03-15', '2026-03-15')
    })

    await act(async () => {
      resolveNewer({ data: [{ id: 'newer' }] })
      await newerRequest
    })
    await act(async () => {
      resolveOlder({ data: [{ id: 'older' }] })
      await olderRequest
    })

    expect(result.current.activities).toEqual([{ id: 'newer' }])
  })
})