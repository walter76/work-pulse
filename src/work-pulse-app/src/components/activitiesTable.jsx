import { useNavigate } from 'react-router-dom'
import { IconButton, Table } from '@mui/joy'
import { Delete, Edit } from '@mui/icons-material'

const ActivitiesTable = ({ activities, categories, onEditActivity, onDeleteActivity }) => {
  const navigate = useNavigate()

  const handleEditActivity = (activity) => {
    if (onEditActivity) {
      onEditActivity(activity)
    } else {
      // Default behavior if no handler is provided
      console.log(`Editing activity with ID: ${activity.id}`)

      navigate(`/activities/edit/${activity.id}`)
    }
  }

  return (
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
        {activities.map((activity) => {
          const category = categories.find((cat) => cat.id === activity.pam_category_id)
          const categoryName = category ? category.name : 'Unknown'

          return (
            <tr key={activity.id}>
              <td>{activity.date}</td>
              <td>{activity.start_time}</td>
              <td>{activity.end_time}</td>
              <td>{categoryName}</td>
              <td>{activity.task}</td>
              <td>
                <IconButton
                  aria-label="Edit Activity"
                  color="primary"
                  variant="soft"
                  onClick={() => handleEditActivity(activity)}
                  size="sm"
                >
                  <Edit />
                </IconButton>
                <IconButton
                  aria-label="Delete Activity"
                  color="danger"
                  variant="soft"
                  onClick={() => onDeleteActivity(activity.id)}
                  size="sm"
                >
                  <Delete />
                </IconButton>
              </td>
            </tr>
          )
        })}
      </tbody>
    </Table>
  )
}

export default ActivitiesTable
