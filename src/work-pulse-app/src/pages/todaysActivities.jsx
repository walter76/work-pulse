import { useEffect, useState } from 'react'
import { Button, Input, Option, Select, Sheet, Table, Typography } from '@mui/joy'
import { Add, Refresh } from '@mui/icons-material'
import axios from 'axios'

const TodaysActivities = () => {
  const today = new Date()
  const formattedDate = today.toISOString().split('T')[0] // Format date as YYYY-MM-DD

  const [activities, setActivities] = useState([])
  const [categories, setCategories] = useState([])
  const [categoryId, setCategoryId] = useState('')
  const [activityDate, setActivityDate] = useState(formattedDate)
  const [startTime, setStartTime] = useState('')
  const [endTime, setEndTime] = useState('')
  const [task, setTask] = useState('')

  useEffect(() => {
    refreshCategories()
    refreshActivities()
  }, [])

  const refreshActivities = async () => {
    console.log('Refreshing activities...')

    try {
      const response = await axios.get(`http://localhost:8080/api/v1/activities?date=${formattedDate}`)

      setActivities(response.data)

      console.log('Activities refreshed successfully!')
    } catch (error) {
      console.error('Error fetching activities:', error)
    }
  }

  const refreshCategories = async () => {
    console.log('Refreshing categories...')

    try {
      const response = await axios.get('http://localhost:8080/api/v1/pam-categories')

      setCategories(response.data)

      console.log('Categories refreshed successfully!')
    } catch (error) {
      console.error('Error fetching categories:', error)
    }
  }

  const createActivity = async () => {
    console.log(`Creating activity for date: ${activityDate}, start time: ${startTime}, end time: ${endTime}, category ID: ${categoryId}, task: ${task}`)

    try {
      await axios.post('http://localhost:8080/api/v1/activities', {
        date: activityDate,
        start_time: startTime,
        end_time: endTime,
        pam_category_id: categoryId,
        task: task
      })

      // Reset the input field after creating the category
      setCategoryId('')
      setActivityDate(formattedDate)
      setStartTime('')
      setEndTime('')
      setTask('')

      // Refresh the activities list after creating the activity
      refreshActivities()

      console.log(`Activity created successfully!`)
    } catch (error) {
      console.error('Error creating activity:', error)
    }    
  }

  return (
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5 }}>
      <Typography level="h2">
        Today's Activities
      </Typography>

      <Sheet variant="outlined" sx={{ display: 'flex', gap: 2, padding: 2 }}>
        <Input
          required
          id="activity-date"
          type="date"
          placeholder="Date"
          value={activityDate}
          onChange={(e) => setActivityDate(e.target.value)}
        />
        <Input
          required
          id="start-time"
          type="time"
          placeholder="Start Time"
          value={startTime}
          onChange={(e) => setStartTime(e.target.value)}
        />
        <Input
          id="end-time"
          type="time"
          placeholder="End Time"
          value={endTime}
          onChange={(e) => setEndTime(e.target.value)}
        />
        <Select
          id="category"
          placeholder="Category"
          value={categoryId}
          onChange={(_e, newValue) => setCategoryId(newValue)}
        >
          {categories.map((category) => (
            <Option key={category.id} value={category.id}>
              {category.name}
            </Option>
          ))}
        </Select>
        <Input
          required
          id="task"
          placeholder="Task"
          value={task}
          onChange={(e) => setTask(e.target.value)}
        />
        <Button startDecorator={<Add/>} onClick={createActivity}>
          Add Activity
        </Button>
      </Sheet>

      <Sheet variant="outlined" sx={{ gap: 2, padding: 2 }}>
        <Typography level="h3">
          Activities List on {formattedDate}
        </Typography>

        <Sheet sx={{ display: 'flex', alignItems: 'center', gap: 2, marginTop: 2, marginBottom: 2 }}>
          <Typography>Number of Records: {activities.length}</Typography>
          <Button startDecorator={<Refresh />} onClick={refreshActivities}>
            Refresh Activities
          </Button>
        </Sheet>

        <Table>
          <thead>
            <tr>
              <th>Date</th>
              <th>Check-In</th>
              <th>Check-Out</th>
              <th>Category</th>
              <th>Task</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {activities
              .sort((a, b) => {
                // first sort by date (descending)
                const dateComparison = new Date(a.date) - new Date(b.date)
                if (dateComparison !== 0) return dateComparison

                // if dates are equal, sort by start time (descending)
                return a.start_time.localeCompare(b.start_time)
              })
              .map((activity) => {
                const category = categories.find((cat) => cat.id === activity.pam_category_id)
                const categoryName = category ? category.name : 'Unknown'

                return (
                  <tr key={activity.id}>
                    <td>{activity.date}</td>
                    <td>{activity.start_time}</td>
                    <td>{activity.end_time}</td>
                    <td>{categoryName}</td>
                    <td>{activity.task}</td>
                  </tr>
                )
            })}
          </tbody>
        </Table>

      </Sheet>

    </Sheet>
  )
}

export default TodaysActivities
