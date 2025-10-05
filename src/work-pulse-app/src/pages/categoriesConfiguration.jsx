import { useEffect, useState } from 'react'
import { Button, IconButton, Input, Sheet, Table, Typography } from '@mui/joy'
import { Add, Check, Close, Delete, Edit, Refresh } from '@mui/icons-material'
import axios from 'axios'

import { API_BASE_URL } from '../config/api'

const CategoriesConfiguration = () => {
  const [categories, setCategories] = useState([])
  const [categoryName, setCategoryName] = useState('')
  const [editingId, setEditingId] = useState(null)
  const [editingName, setEditingName] = useState('')
  const [error, setError] = useState('')

  useEffect(() => {
    refreshCategories()
  }, [])
  
  const refreshCategories = async () => {
    console.log('Refreshing categories...')
    setError('')

    try {
      const response = await axios.get(`${API_BASE_URL}/api/v1/pam-categories`)

      setCategories(response.data)

      console.log('Categories refreshed successfully!')
    } catch (error) {
      console.error('Error fetching categories:', error)
      setError('Failed to fetch categories. Please try again.')
    }
  }

  const createCategory = async () => {
    if (!categoryName) {
      setError('Please enter a category name.')
      return
    }

    // Here you would typically make an API call to create the category
    console.log(`Creating category: ${categoryName}`)
    setError('')

    try {
      await axios.post(`${API_BASE_URL}/api/v1/pam-categories`, {
        name: categoryName,
      })

      // Reset the input field after creating the category
      setCategoryName('')

      // Refresh the categories list after creation
      refreshCategories()

      console.log(`Category "${categoryName}" created successfully!`)
    } catch (error) {
      console.error('Error creating category:', error)
      setError('Failed to create category. Is the category name already existing? Please try again.')
    }
  }

  const deleteCategory = async (categoryId) => {
    console.log(`Deleting category with ID: ${categoryId}`)
    setError('')

    try {
      await axios.delete(`${API_BASE_URL}/api/v1/pam-categories/${categoryId}`)
    
      // Refresh the categories list after deletion
      refreshCategories()
    
      console.log(`Category with ID ${categoryId} deleted successfully!`)
    } catch (error) {
      console.error(`Error deleting category with ID ${categoryId}:`, error)
      setError('Failed to delete category. Please try again.')
    }
  }

  const startEditing = (category) => {
    setEditingId(category.id)
    setEditingName(category.name)
  }

  const cancelEditing = () => {
    setEditingId(null)
    setEditingName('')
  }

  const saveCategory = async () => {
    console.log(`Renaming category with ID: ${editingId} to "${editingName}"`)
    setError('')

    try {
      await axios.put(`${API_BASE_URL}/api/v1/pam-categories`, {
        id: editingId,
        name: editingName,
      })

      setEditingId(null)
      setEditingName('')
      refreshCategories()
    } catch (error) {
        console.error(`Error renaming category:`, error)
        setError('Failed to rename category. Is the category name already existing? Please try again.')
    }
  }

  return (
    <Sheet sx={{ display: 'flex', flexDirection: 'column', gap: 5 }}>
      <Typography level="h2">
        Categories Configuration
      </Typography>

      {error && (
        <Typography level="body-md" color="danger" sx={{ padding: 1 }}>
          {error}
        </Typography>
      )}
      
      <Sheet variant="outlined" sx={{ display: 'flex', gap: 2, padding: 2 }}>
        <Input
          required
          id="category-name"
          size="sm"
          placeholder="Category Name"
          value={categoryName}
          onChange={(e) => setCategoryName(e.target.value)}
        />
        <Button startDecorator={<Add/>} size="sm" onClick={createCategory}>
          Add Category
        </Button>
      </Sheet>

      <Sheet variant="outlined" sx={{ gap: 2, padding: 2 }}>
        <Typography level="h3">
          Categories List
        </Typography>

        <Sheet sx={{ display: 'flex', alignItems: 'center', gap: 2, marginTop: 2, marginBottom: 2 }}>
          <Typography>Number of Records: {categories.length}</Typography>
          <Button startDecorator={<Refresh />} size="sm" onClick={refreshCategories}>
            Refresh Categories
          </Button>
        </Sheet>

        <Table>
          <thead>
            <tr>
              <th>Category Name</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {categories.map((category) => (
              <tr key={category.id}>
                <td>
                  {editingId === category.id ? (
                    <Input
                      value={editingName}
                      onChange={ e => setEditingName(e.target.value) }
                      size="sm"
                      autoFocus
                      onKeyDown={ e => {
                        if (e.key === 'Enter') saveCategory()
                        if (e.key === 'Escape') cancelEditing()
                      }}
                      sx={{ minWidth: 150}}
                    />
                  ) : (
                    category.name
                  )}
                </td>
                <td>
                  {editingId === category.id ? (
                    <>
                      <IconButton 
                        aria-label="Save"
                        color="success"
                        variant="soft"
                        onClick={saveCategory}
                        size="sm"
                        sx={{ mr: 1 }}
                      >
                        <Check />
                      </IconButton>
                      <IconButton 
                        aria-label="Cancel"
                        color="neutral"
                        variant="soft"
                        onClick={cancelEditing}
                        size="sm"
                      >
                        <Close />
                      </IconButton>
                    </>
                  ) : (
                    <>
                      <IconButton 
                        aria-label="Rename Category"
                        color="primary"
                        variant="soft"
                        onClick={() => startEditing(category)}
                        size="sm"
                        sx={{ mr: 1 }}
                      >
                        <Edit />
                      </IconButton>
                      <IconButton 
                        aria-label="Delete Category"
                        color="danger"
                        variant="soft"
                        onClick={() => deleteCategory(category.id)}
                        size="sm"
                      >
                        <Delete />
                      </IconButton>
                    </>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </Table>
      </Sheet>
    </Sheet>
  )
}

export default CategoriesConfiguration
