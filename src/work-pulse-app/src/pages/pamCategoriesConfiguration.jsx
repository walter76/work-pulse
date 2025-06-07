import { useState } from 'react'
import { Button, Input, Sheet, Table, Typography } from '@mui/joy'
import axios from 'axios'

const PamCategoriesConfiguration = () => {
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
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5, margin: 5 }}>
      <Typography level="h2">
        PAM Categories Configuration
      </Typography>

      <Sheet variant="outlined" sx={{ display: 'flex', gap: 2, padding: 2 }}>
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
      </Sheet>

      <Sheet variant="outlined" sx={{ gap: 2, padding: 2 }}>
        <Typography level="h3">
          Categories List
        </Typography>

        <Sheet sx={{ display: 'flex', alignItems: 'center', gap: 2, marginTop: 2, marginBottom: 2 }}>
          <Typography>Number of Records: {categories.length}</Typography>
          <Button onClick={refreshCategories}>
            Refresh Categories
          </Button>
        </Sheet>

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
      </Sheet>
    </Sheet>
  )
}

export default PamCategoriesConfiguration
