import { useNavigate } from 'react-router-dom'
import { IconButton, Table, Typography } from '@mui/joy'
import { Delete, Edit } from '@mui/icons-material'

import {
  calculateDurationInMinutes,
  formatDateForDisplay,
  formatDuration,
} from '../lib/dateUtils'

/**
 * Renders activity records with their duration and an optional daily total.
 * @param {Object} props - Component inputs.
 * @param {Array<Object>} props.activities - Activity records to render.
 * @param {Array<Object>} props.categories - Available accounting categories.
 * @param {Function} props.onEditActivity - Edit action callback.
 * @param {Function} props.onDeleteActivity - Delete action callback.
 * @param {string} [props.totalDuration] - Optional formatted daily total.
 * @param {string} [props.totalDurationColor] - Optional Joy UI color for the daily total.
 * @returns {JSX.Element} The activity table.
 */
const ActivitiesTable = ({
  activities,
  categories,
  onEditActivity,
  onDeleteActivity,
  totalDuration,
  totalDurationColor,
}) => {
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
          <th>Duration</th>
          <th>Category</th>
          <th>Task</th>
          <th>Comment</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {activities.map((activity) => {
          const category = categories.find((cat) => cat.id === activity.accounting_category_id)
          const categoryName = category ? category.name : 'Unknown'

          return (
            <tr key={activity.id}>
              <td>{formatDateForDisplay(activity.date)}</td>
              <td>{activity.start_time}</td>
              <td>{activity.end_time}</td>
              <td>
                {formatDuration(
                  `PT${calculateDurationInMinutes(activity.start_time, activity.end_time)}M`,
                )}
              </td>
              <td>{categoryName}</td>
              <td>{activity.task}</td>
              <td>{activity.comment}</td>
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
      {totalDuration !== undefined && (
        <tfoot>
          <tr>
            <td colSpan={3}>Daily total</td>
            <td>
              <Typography color={totalDurationColor} fontWeight="bold">
                {totalDuration}
              </Typography>
            </td>
            <td colSpan={4}></td>
          </tr>
        </tfoot>
      )}
    </Table>
  )
}

export default ActivitiesTable
