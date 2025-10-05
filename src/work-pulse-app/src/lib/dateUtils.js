// Helper function to get week number from a date
export const getWeekNumber = (date) => {
  const d = new Date(date)
  d.setHours(0, 0, 0, 0)
  d.setDate(d.getDate() + 3 - ((d.getDay() + 6) % 7))
  const week1 = new Date(d.getFullYear(), 0, 4)
  return 1 + Math.round(((d - week1) / 86400000 - 3 + ((week1.getDay() + 6) % 7)) / 7)
}

// Helper function to get the start and end dates of the week for a given date (Monday to Sunday)
export const getWeekRange = (date) => {
  const d = new Date(date)
  const day = d.getDay()
  const diffToMonday = d.getDate() - day + (day === 0 ? -6 : 1)
  const monday = new Date(d.setDate(diffToMonday))
  const sunday = new Date(monday)
  sunday.setDate(monday.getDate() + 6)

  return {
    start: monday.toISOString().split('T')[0],
    end: sunday.toISOString().split('T')[0],
  }
}

// Function to group activities by week
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

// Function to get the first and last day of the current month
export const getCurrentMonthRange = () => {
  const now = new Date()
  const firstDay = new Date(now.getFullYear(), now.getMonth(), 1)
  const lastDay = new Date(now.getFullYear(), now.getMonth() + 1, 0)

  return {
    start: firstDay.toISOString().split('T')[0],
    end: lastDay.toISOString().split('T')[0],
  }
}
