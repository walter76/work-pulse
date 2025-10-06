import { getWeekNumber } from './dateUtils'

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
                '2025-01-01', '2025-03-15', '2025-06-30',
                '2025-09-15', '2025-12-31', '2024-12-30'
            ]

            testDates.forEach(date => {
                const weekNumber = getWeekNumber(date)
                expect(weekNumber).toBeGreaterThanOrEqual(1)
                expect(weekNumber).toBeLessThanOrEqual(53)
            })
        })
    })
})