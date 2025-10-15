import { useState } from 'react'
import { Box, Button, Input, Sheet, Typography } from '@mui/joy'
import { CloudUpload } from '@mui/icons-material'
import axios from 'axios'

import ErrorMessage from '../components/errorMessage'

import { API_BASE_URL } from '../config/api'

const ImportActivities = () => {
  const [selectedYear, setSelectedYear] = useState(new Date().getFullYear().toString())
  const [selectedFile, setSelectedFile] = useState(null)
  const [error, setError] = useState('')
  const [success, setSuccess] = useState('')

  const handleFileChange = (event) => {
    const file = event.target.files[0]
    setSelectedFile(file)
    setError('')
    setSuccess('')
  }

  const handleImport = async () => {
    if (!selectedYear) {
      setError('Please specify a valid year for import.')
      return
    }

    if (!selectedFile) {
      setError('Please select a file to import.')
      return
    }

    setError('')
    setSuccess('')

    try {
      const formData = new FormData()
      formData.append('file', selectedFile)

      // Add your import logic here, e.g., sending the file to the backend
      console.log(`Importing activities for year ${selectedYear} from file:`, selectedFile.name)

      const response = await axios.post(
        `${API_BASE_URL}/api/v1/activities/upload-csv?activities_year=${selectedYear}`,
        formData,
        {
          headers: {
            'Content-Type': 'multipart/form-data',
          },
        },
      )

      setSuccess('Activities imported successfully!')
      setSelectedFile(null)

      // Reset file input
      const fileInput = document.getElementById('file-upload')
      if (fileInput) {
        fileInput.value = ''
      }

      console.log('Import response:', response.data)
    } catch (error) {
      console.error('Error importing activities:', error)
      setError('Failed to import activities. Please try again.')
    }
  }

  return (
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5 }}>
      <Typography level="h2">Import Activities</Typography>

      <ErrorMessage message={error} />

      {success && (
        <Typography level="body-md" color="success" sx={{ padding: 1 }}>
          {success}
        </Typography>
      )}

      <Sheet variant="outlined" sx={{ padding: 3, gap: 3 }}>
        <Typography level="h3">Import Configuration</Typography>

        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
            <Typography level="body-sm" fontWeight="bold">
              Import Year:
            </Typography>

            <Input
              required
              type="number"
              placeholder="Year (e.g., 2024)"
              value={selectedYear}
              onChange={(e) => setSelectedYear(e.target.value)}
              sx="{{ maxWidth: 200 }}"
              min="2000"
              max="2099"
            />
          </Box>

          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
            <Typography level="body-sm" fontWeight="bold">
              Select File:
            </Typography>

            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
              <Input
                required
                id="file-upload"
                type="file"
                onChange={handleFileChange}
                accept=".csv"
                sx={{ flex: 1 }}
              />
              <Typography level="body-xs" color="neutral">
                Supported formats: CSV
              </Typography>
            </Box>

            {selectedFile && (
              <Typography level="body-sm" color="primary">
                Selected File: {selectedFile.name}
              </Typography>
            )}
          </Box>
        </Box>

        <Box sx={{ display: 'flex', gap: 2, marginTop: 2 }}>
          <Button
            startDecorator={<CloudUpload />}
            onClick={handleImport}
            disabled={!selectedFile || !selectedYear}
          >
            Import Activities
          </Button>
        </Box>
      </Sheet>

      <Sheet variant="outlined" sx={{ padding: 3 }}>
        <Typography level="h3" sx={{ marginBottom: 2 }}>
          File Format Requirements
        </Typography>
        <Typography level="body-sm" sx={{ marginBottom: 1 }}>
          Tue uploaded file should contain the following columns:
        </Typography>
        <ul style={{ margin: 0, paddingLeft: '20px' }}>
          <li>
            <Typography level="body-sm">CW (integer)</Typography>
          </li>
          <li>
            <Typography level="body-sm">Date (format: DD.MM.)</Typography>
          </li>
          <li>
            <Typography level="body-sm">Check In (format: HH:MM)</Typography>
          </li>
          <li>
            <Typography level="body-sm">Check Out (format: HH:MM)</Typography>
          </li>
          <li>
            <Typography level="body-sm">Duration (format: HH:MM:SS)</Typography>
          </li>
          <li>
            <Typography level="body-sm">PAM Category (string)</Typography>
          </li>
          <li>
            <Typography level="body-sm">Topic (string)</Typography>
          </li>
          <li>
            <Typography level="body-sm">Comment (string)</Typography>
          </li>
        </ul>
        <Typography level="body-sm" sx={{ marginTop: 1 }}>
          Ensure that the date and time formats are strictly followed to avoid import errors.
        </Typography>
      </Sheet>
    </Sheet>
  )
}

export default ImportActivities
