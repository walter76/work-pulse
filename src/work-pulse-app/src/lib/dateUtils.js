/**
 * Calculates the ISO week number for a given date.
 * Week 1 is the first week that contains at least 4 days of the new year.
 *
 * @param {string|Date} date - The date to calculate the week number for (YYYY-MM-DD format or Date object).
 * @returns {number} - The ISO week number (1-53).
 *
 * @example
 * getWeekNumber('2025-01-06') // Returns 2
 * getWeekNumber(new Date('2025-01-01')) // Returns 1
 */
export const getWeekNumber = (date) => {
  // create a new Date object and normalize to midnight
  const targetDate = new Date(date)
  targetDate.setHours(0, 0, 0, 0)

  // Find the Thursday of the week containing the target date
  // This is used because ISO weeks are defined by their Thursdays
  const dayOfWeek = targetDate.getDay()
  const daysSinceMonday = (dayOfWeek + 6) % 7 // Convert Sunday=0 to Monday=0 system
  const daysToThursday = 3 - daysSinceMonday

  const thursday = new Date(targetDate)
  thursday.setDate(targetDate.getDate() + daysToThursday)

  // Find January 4th of the same year as Thursday
  // This date is always in week 1 according to ISO standard
  const january4th = new Date(thursday.getFullYear(), 0, 4)

  // Find the Thursday of the week containing January 4th
  const jan4DayOfWeek = january4th.getDay()
  const jan4DaysSinceMonday = (jan4DayOfWeek + 6) % 7
  const jan4DaysToThursday = 3 - jan4DaysSinceMonday

  const firstThursday = new Date(january4th)
  firstThursday.setDate(january4th.getDate() + jan4DaysToThursday)

  // Calculate the difference in weeks between the two Thursdays
  const millisecondsPerDay = 86400000
  const daysDifference = (thursday.getTime() - firstThursday.getTime()) / millisecondsPerDay
  const weeksDifference = Math.round(daysDifference / 7)

  // Week 1 + the number of weeks difference
  return 1 + weeksDifference
}

/**
 * Gets the start (Monday) and end (Sunday) dates of the week containing the given date.
 *
 * @param {string|Date} date - The date to find the week range for (YYYY-MM-DD format or Date object).
 * @returns {Object} An object containing start and end dates in YYYY-MM-DD format.
 * @returns {string} returns.start - The Monday of the week (YYYY-MM-DD).
 * @returns {string} returns.end - The Sunday of the week (YYYY-MM-DD).
 *
 * @example
 * getWeekRange('2025-01-08') // Returns { start: '2025-01-06', end: '2025-01-12' }
 */
export const getWeekRange = (date) => {
  // Create a new Date object to avoid mutating the original
  const targetDate = new Date(date)

  // Get the day of the week (0=Sunday, 1=Monday, ..., 6=Saturday)
  const dayOfWeek = targetDate.getDay()

  // Calculate how many days to subtract to get to Monday
  // If it's Sunday (0), we need to go back 6 days
  // If it's Monday (1), we need to go back 0 days
  // If it's Tuesday (2), we need to go back 1 day, etc.
  const daysToSubtractForMonday = dayOfWeek === 0 ? 6 : dayOfWeek - 1

  // Find the Monday of this week
  const monday = new Date(targetDate)
  monday.setDate(targetDate.getDate() - daysToSubtractForMonday)

  // Find the Sunday of this week (6 days after Monday)
  const sunday = new Date(monday)
  sunday.setDate(monday.getDate() + 6)

  // Return dates in YYYY-MM-DD format
  return {
    start: monday.toISOString().split('T')[0],
    end: sunday.toISOString().split('T')[0],
  }
}

/**
 * Groups and array of activities by their calendar week.
 * Each group contains the week number, year, week range, and activities for that week.
 *
 * @param {Array<Object>} activities - Array of activity objects
 * @param {string} activities[].date - Activity date in YYYY-MM-DD format.
 * @param {string} activities[].start_time - Activity start time in HH:MM format.
 * @param {string} activities[].end_time - Activity end time in HH:MM format.
 * @param {number} activities[].pam_category_id - ID of the activity category.
 * @param {string} activities[].task - Description of the activity task.
 * @returns {Object} An object where keys are week identifiers (e.g., "2025-W01") and values are objects containing week details and activities.
 *
 * @example
 * const activities = [
 *  { date: '2025-01-06', start_time: '09:00', end_time: '10:00', pam_category_id: 1, task: 'Task A' },
 *  { date: '2025-01-07', start_time: '10:00', end_time: '11:00', pam_category_id: 1, task: 'Task B' },
 *  { date: '2025-01-08', start_time: '11:00', end_time: '12:00', pam_category_id: 2, task: 'Task C' },
 * ]
 * groupActivitiesByWeek(activities)
 * // Returns:
 * // {
 * //   '2025-W01': {
 * //     weekNumber: 1,
 * //     year: 2025,
 * //     weekRange: { start: '2025-01-01', end: '2025-01-07' },
 * //     activities: [
 * //       { date: '2025-01-06', start_time: '09:00', end_time: '10:00', pam_category_id: 1, task: 'Task A' },
 * //       { date: '2025-01-07', start_time: '10:00', end_time: '11:00', pam_category_id: 1, task: 'Task B' },
 * //     ]
 * //   },
 * //   '2025-W02': {
 * //     weekNumber: 2,
 * //     year: 2025,
 * //     weekRange: { start: '2025-01-08', end: '2025-01-14' },
 * //     activities: [
 * //       { date: '2025-01-08', start_time: '11:00', end_time: '12:00', pam_category_id: 2, task: 'Task C' },
 * //     ]
 * //   }
 * // }
 */
export const groupActivitiesByWeek = (activities) => {
  const grouped = {}

  activities.forEach((activity) => {
    const weekNumber = getWeekNumber(activity.date)
    const year = new Date(activity.date).getFullYear()
    const weekKey = `${year}-W${weekNumber.toString().padStart(2, '0')}`
    const weekRange = getWeekRange(activity.date)

    if (!grouped[weekKey]) {
      grouped[weekKey] = {
        weekNumber,
        year,
        weekRange,
        activities: [],
      }
    }

    grouped[weekKey].activities.push(activity)
  })

  return grouped
}

/**
 * 
 * Gets the first and last day of the current month in YYYY-MM-DD format.
 * 
 * @returns {Object} An object containing the start and end dates of the current month.
 * @returns {string} returns.start - The first day of the current month (YYYY-MM-DD).
 * @returns {string} returns.end - The last day of the current month (YYYY-MM-DD).
 * 
 * @example
 * // If current date is 2025-10-15
 * getCurrentMonthRange()
 * // Returns:
 * // {
 * //   start: '2025-10-01',
 * //   end: '2025-10-31'
 * // }
 */
export const getCurrentMonthRange = () => {
  const now = new Date()
  const firstDay = new Date(now.getFullYear(), now.getMonth(), 1)
  const lastDay = new Date(now.getFullYear(), now.getMonth() + 1, 0)

  return {
    start: firstDay.toISOString().split('T')[0],
    end: lastDay.toISOString().split('T')[0],
  }
}
