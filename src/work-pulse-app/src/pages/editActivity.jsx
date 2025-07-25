import { useState } from 'react'
import { Sheet, Typography } from '@mui/joy'

const EditActivity = ({ activity }) => {
    const [error, setError] = useState('')

    return (
        <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5 }}>
          <Typography level="h2">
            Today's Activities
          </Typography>
    
          {error && (
            <Typography level="body-md" color="danger" sx={{ padding: 1 }}>
              {error}
            </Typography>
          )}
        </Sheet>
    )    
}

export default EditActivity
