import { Typography } from '@mui/joy'

const ErrorMessage = ({ message }) => {
  return (
    <>
      {message && (
        <Typography level="body-md" color="danger" sx={{ padding: 1 }}>
          {message}
        </Typography>
      )}
    </>
  )
}

export default ErrorMessage
