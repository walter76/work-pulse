import { useState } from 'react'
import { Button, IconButton, Input, Sheet, Table, Typography } from '@mui/joy'
import { Add, Check, Close, Delete, Edit, Refresh } from '@mui/icons-material'
import axios from 'axios'

const PamCategoriesConfiguration = () => {
  const [categories, setCategories] = useState([])
  const [categoryName, setCategoryName] = useState('')
  const [editingId, setEditingId] = useState(null)
  const [editingName, setEditingName] = useState('')

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
      await axios.post('http://localhost:8080/api/v1/pam-categories', {
        name: categoryName,
      })

      // Reset the input field after creating the category
      setCategoryName('')

      // Refresh the categories list after creation
      refreshCategories()

      console.log(`Category "${categoryName}" created successfully!`)
    } catch (error) {
      console.error('Error creating category:', error)
    }
  }

  const deleteCategory = async (categoryId) => {
    console.log(`Deleting category with ID: ${categoryId}`)

    try {
      await axios.delete(`http://localhost:8080/api/v1/pam-categories/${categoryId}`)
    
      // Refresh the categories list after deletion
      refreshCategories()
    
      console.log(`Category with ID ${categoryId} deleted successfully!`)
    } catch (error) {
      console.error(`Error deleting category with ID ${categoryId}:`, error)
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

    try {
      await axios.put(`http://localhost:8080/api/v1/pam-categories`, {
        id: editingId,
        name: editingName,
      })

      setEditingId(null)
      setEditingName('')
      refreshCategories()
    } catch (error) {
        console.error(`Error renaming category:`, error)
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
        <Button startDecorator={<Add/>} onClick={createCategory}>
          Add Category
        </Button>
      </Sheet>

      <Sheet variant="outlined" sx={{ gap: 2, padding: 2 }}>
        <Typography level="h3">
          Categories List
        </Typography>

        <Sheet sx={{ display: 'flex', alignItems: 'center', gap: 2, marginTop: 2, marginBottom: 2 }}>
          <Typography>Number of Records: {categories.length}</Typography>
          <Button startDecorator={<Refresh />} onClick={refreshCategories}>
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
                        aria-label="Rename PAM Category"
                        color="primary"
                        variant="soft"
                        onClick={() => startEditing(category)}
                        size="sm"
                        sx={{ mr: 1 }}
                      >
                        <Edit />
                      </IconButton>
                      <IconButton 
                        aria-label="Delete PAM Category"
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

export default PamCategoriesConfiguration
