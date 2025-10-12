import { formatDuration, getWeekNumber, getWeekRange } from './dateUtils'

describe('getWeekNumber', () => {
  describe('accepts different input formats', () => {
    it('should accept string date in YYYY-MM-DD format', () => {
      const result = getWeekNumber('2025-01-06')
      expect(result).toBe(2)
    })

    it('should accept Date object', () => {
      const result = getWeekNumber(new Date('2025-01-06'))
      expect(result).toBe(2)
    })
  })

  describe('calculates correct week numbers for 2025', () => {
    it('should return week 1 for January 1st, 2025 (Wednesday)', () => {
      expect(getWeekNumber('2025-01-01')).toBe(1)
    })

    it('should return week 1 for January 5th, 2025 (Sunday)', () => {
      expect(getWeekNumber('2025-01-05')).toBe(1)
    })

    it('should return week 2 for January 6th, 2025 (Monday)', () => {
      expect(getWeekNumber('2025-01-06')).toBe(2)
    })

    it('should return week 2 for January 12th, 2025 (Sunday)', () => {
      expect(getWeekNumber('2025-01-12')).toBe(2)
    })
  })

  describe('handles ISO week year boundaries correctly', () => {
    it('should return week 52 for December 29th, 2024 (Sunday)', () => {
      expect(getWeekNumber('2024-12-29')).toBe(52)
    })

    it('should return week 1 for December 30th, 2024 (Monday) - belongs to 2025', () => {
      expect(getWeekNumber('2024-12-30')).toBe(1)
    })

    it('should return week 1 for December 31st, 2024 (Tuesday) - belongs to 2025', () => {
      expect(getWeekNumber('2024-12-31')).toBe(1)
    })
  })

  describe('handles edge cases for different years', () => {
    it('should return correct week for January 4th (always week 1)', () => {
      expect(getWeekNumber('2023-01-04')).toBe(1)
      expect(getWeekNumber('2024-01-04')).toBe(1)
      expect(getWeekNumber('2025-01-04')).toBe(1)
      expect(getWeekNumber('2026-01-04')).toBe(1)
    })

    it('should handle leap year correctly', () => {
      expect(getWeekNumber('2024-02-29')).toBe(9)
      expect(getWeekNumber('2024-12-30')).toBe(1) // Dec 30, 2024 is a Monday, belongs to week 1 of 2025
    })

    it('should handle non-leap year correctly', () => {
      expect(getWeekNumber('2023-02-28')).toBe(9)
      expect(getWeekNumber('2023-12-31')).toBe(52)
    })
  })

  describe('handles mid-year dates correctly', () => {
    it('should return correct week for mid-year dates', () => {
      expect(getWeekNumber('2025-06-15')).toBe(24) // June 15, 2025
      expect(getWeekNumber('2025-07-01')).toBe(27) // July 1, 2025
      expect(getWeekNumber('2025-12-01')).toBe(49) // December 1, 2025
    })
  })

  describe('handles all days of the week consistently', () => {
    // Week of January 6-12, 2025 (Week 2)
    it('should return week 2 for each day from January 6 to January 12, 2025', () => {
      expect(getWeekNumber('2025-01-06')).toBe(2) // Monday
      expect(getWeekNumber('2025-01-07')).toBe(2) // Tuesday
      expect(getWeekNumber('2025-01-08')).toBe(2) // Wednesday
      expect(getWeekNumber('2025-01-09')).toBe(2) // Thursday
      expect(getWeekNumber('2025-01-10')).toBe(2) // Friday
      expect(getWeekNumber('2025-01-11')).toBe(2) // Saturday
      expect(getWeekNumber('2025-01-12')).toBe(2) // Sunday
    })
  })

  describe('returns valid week numbers', () => {
    it('should return week numbers between 1 and 53', () => {
      const testDates = [
        '2025-01-01',
        '2025-03-15',
        '2025-06-30',
        '2025-09-15',
        '2025-12-31',
        '2024-12-30',
      ]

      testDates.forEach((date) => {
        const weekNumber = getWeekNumber(date)
        expect(weekNumber).toBeGreaterThanOrEqual(1)
        expect(weekNumber).toBeLessThanOrEqual(53)
      })
    })
  })
})

describe('getWeekRange', () => {
  describe('accepts different input formats', () => {
    it('should accept string date in YYYY-MM-DD format', () => {
      const result = getWeekRange('2025-01-08')
      expect(result).toEqual({
        start: '2025-01-06',
        end: '2025-01-12',
      })
    })

    it('should accept Date object', () => {
      const result = getWeekRange(new Date('2025-01-08'))
      expect(result).toEqual({
        start: '2025-01-06',
        end: '2025-01-12',
      })
    })
  })

  describe('calculates correct week ranges for different days of the week', () => {
    // Test week: January 6-12, 2025
    it('should return correct range when input is Monday', () => {
      const result = getWeekRange('2025-01-06') // Monday
      expect(result).toEqual({
        start: '2025-01-06',
        end: '2025-01-12',
      })
    })

    it('should return correct range when input is Wednesday', () => {
      const result = getWeekRange('2025-01-08') // Wednesday
      expect(result).toEqual({
        start: '2025-01-06',
        end: '2025-01-12',
      })
    })

    it('should return correct range when input is Friday', () => {
      const result = getWeekRange('2025-01-10') // Friday
      expect(result).toEqual({
        start: '2025-01-06',
        end: '2025-01-12',
      })
    })

    it('should return correct range when input is Sunday', () => {
      const result = getWeekRange('2025-01-12') // Sunday
      expect(result).toEqual({
        start: '2025-01-06',
        end: '2025-01-12',
      })
    })
  })

  describe('handles all days of the same week consistently', () => {
    it('should return the same week range for all days in January 6-12, 2025', () => {
      const expectedRange = {
        start: '2025-01-06',
        end: '2025-01-12',
      }

      expect(getWeekRange('2025-01-06')).toEqual(expectedRange) // Monday
      expect(getWeekRange('2025-01-07')).toEqual(expectedRange) // Tuesday
      expect(getWeekRange('2025-01-08')).toEqual(expectedRange) // Wednesday
      expect(getWeekRange('2025-01-09')).toEqual(expectedRange) // Thursday
      expect(getWeekRange('2025-01-10')).toEqual(expectedRange) // Friday
      expect(getWeekRange('2025-01-11')).toEqual(expectedRange) // Saturday
      expect(getWeekRange('2025-01-12')).toEqual(expectedRange) // Sunday
    })
  })

  describe('handles month boundaries correctly', () => {
    it('should handle week spanning across months', () => {
      const result = getWeekRange('2025-02-01') // Saturday
      expect(result).toEqual({
        start: '2025-01-27', // Monday of previous month
        end: '2025-02-02', // Sunday of current month
      })
    })

    it('should handle end of month dates', () => {
      const result = getWeekRange('2025-01-31') // Friday
      expect(result).toEqual({
        start: '2025-01-27',
        end: '2025-02-02',
      })
    })
  })

  describe('handles year boundaries correctly', () => {
    it('should handle week spanning across years', () => {
      const result = getWeekRange('2024-12-30') // Monday
      expect(result).toEqual({
        start: '2024-12-30', // Monday of current year
        end: '2025-01-05', // Sunday of next year
      })
    })

    it('should handle New Year Day', () => {
      const result = getWeekRange('2025-01-01') // Wednesday
      expect(result).toEqual({
        start: '2024-12-30', // Monday of previous year
        end: '2025-01-05', // Sunday of current year
      })
    })

    it('should handle last day of year', () => {
      const result = getWeekRange('2024-12-31') // Tuesday
      expect(result).toEqual({
        start: '2024-12-30', // Monday of current year
        end: '2025-01-05', // Sunday of next year
      })
    })
  })

  describe('handles leap year correctly', () => {
    it('should handle February 29th in a leap year', () => {
      const result = getWeekRange('2024-02-29') // Thursday
      expect(result).toEqual({
        start: '2024-02-26',
        end: '2024-03-03',
      })
    })
  })

  describe('handles various dates throughout the year', () => {
    it('should handle mid-year dates correctly', () => {
      const result1 = getWeekRange('2025-06-15') // Sunday
      expect(result1).toEqual({
        start: '2025-06-09',
        end: '2025-06-15',
      })

      const result2 = getWeekRange('2025-09-10') // Wednesday
      expect(result2).toEqual({
        start: '2025-09-08',
        end: '2025-09-14',
      })
    })
  })

  describe('return format validation', () => {
    it('should return object with start and end properties', () => {
      const result = getWeekRange('2025-01-08')

      expect(result).toHaveProperty('start')
      expect(result).toHaveProperty('end')
      expect(typeof result.start).toBe('string')
      expect(typeof result.end).toBe('string')
    })

    it('should return dates in YYYY-MM-DD format', () => {
      const result = getWeekRange('2025-01-08')

      // Check format with regex: YYYY-MM-DD
      const dateRegex = /^\d{4}-\d{2}-\d{2}$/
      expect(result.start).toMatch(dateRegex)
      expect(result.end).toMatch(dateRegex)
    })

    it('should ensure start date is always before end date', () => {
      const testDates = ['2025-01-01', '2025-03-15', '2025-06-30', '2025-09-15', '2025-12-31']

      testDates.forEach((date) => {
        const result = getWeekRange(date)
        expect(new Date(result.start) <= new Date(result.end)).toBe(true)
      })
    })

    it('should ensure week is exactly 7 days long', () => {
      const testDates = ['2025-01-01', '2025-03-15', '2025-06-30', '2025-09-15', '2025-12-31']

      testDates.forEach((date) => {
        const result = getWeekRange(date)
        const start = new Date(result.start)
        const end = new Date(result.end)
        const diffTime = Math.abs(end - start)
        const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24)) + 1 // +1 to include both start and end dates
        expect(diffDays).toBe(7)
      })
    })
  })

  describe('edge cases', () => {
    it('should handle dates at the beginning of time', () => {
      const result = getWeekRange('1970-01-01') // Thursday
      expect(result).toEqual({
        start: '1969-12-29',
        end: '1970-01-04',
      })
    })

    it('should handle future dates', () => {
      const result = getWeekRange('2100-12-31') // Friday
      expect(result).toEqual({
        start: '2100-12-27',
        end: '2101-01-02',
      })
    })
  })
})

describe('formatDuration', () => {
  it('should format duration correctly for standard cases', () => {
    expect(formatDuration('PT10800S')).toBe('03:00')
  })
})
