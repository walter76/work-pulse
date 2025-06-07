import { useState } from 'react'
import { Box, Button, Card, CardContent, Input, Table, Typography } from '@mui/joy'
import axios from 'axios'

const App = () => {
  const [categories, setCategories] = useState([])
  const [categoryName, setCategoryName] = useState('')

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

  const createCategory = async () => {
    if (!categoryName) {
      alert('Please enter a category name.')
      return
    }

    // Here you would typically make an API call to create the category
    console.log(`Creating category: ${categoryName}`)

    try {
      const response = await axios.post('http://localhost:8080/api/v1/pam-categories', {
        id: '',
        name: categoryName,
      })

      // Reset the input field after creating the category
      setCategoryName('')

      console.log(`Category "${categoryName}" created successfully!`)
    } catch (error) {
      console.error('Error creating category:', error)
    }
  }

  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', gap: 5, margin: 5 }}>
      <Typography level="h2">
        PAM Categories Configuration
      </Typography>

      <Card variant="outlined" sx={{ margin: 2 }}>
        <CardContent>
          <Box sx={{ display: 'flex', gap: 2 }}>
            <Input
              required
              id="category-name"
              placeholder="Category Name"
              value={categoryName}
              onChange={(e) => setCategoryName(e.target.value)}
            />
            <Button onClick={createCategory}>
              Add Category
            </Button>
          </Box>
        </CardContent>
      </Card>

      <Card variant="outlined" sx={{ margin: 2 }}>
        <CardContent>
          <Typography level="h3">
            Categories List
          </Typography>
          <Box sx={{ display: 'flex', justifyContent: 'space-between', marginBottom: 2}}>
            <Typography>Number of Records: {categories.length}</Typography>
            <Button onClick={refreshCategories}>
              Refresh Categories
            </Button>
          </Box>
          <Table>
            <thead>
              <tr>
                <th>Category Name</th>
              </tr>
            </thead>
            <tbody>
              {categories.map((category) => (
                <tr key={category.id}>
                  <td>{category.name}</td>
                </tr>
              ))}
            </tbody>
          </Table>
        </CardContent>
      </Card>
    </Box>
  )
}

export default App
