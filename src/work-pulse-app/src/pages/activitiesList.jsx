import { useEffect, useState } from 'react'
import { Button, Input, Option, Select, Sheet, Typography } from '@mui/joy'
import { Add } from '@mui/icons-material'
import axios from 'axios'

const ActivitiesList = () => {
  const [categories, setCategories] = useState([])
  const [categoryId, setCategoryId] = useState('')
  const [activityDate, setActivityDate] = useState('')
  const [startTime, setStartTime] = useState('')
  const [endTime, setEndTime] = useState('')
  const [task, setTask] = useState('')

  useEffect(() => {
    // Fetch categories when the component mounts
    refreshCategories()
  }, [])

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
      setActivityDate('')
      setStartTime('')
      setEndTime('')
      setTask('')

      console.log(`Activity created successfully!`)
    } catch (error) {
      console.error('Error creating activity:', error)
    }    
  }

  return (
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5, margin: 5 }}>
      <Typography level="h2">
        Activities
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
          id="pam-category"
          placeholder="PAM Category"
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
    </Sheet>
  )
}

export default ActivitiesList
