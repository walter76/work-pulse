import { Sheet } from '@mui/joy'

import ActivitiesList from "../pages/activitiesList"
import PamCategoriesConfiguration from "../pages/pamCategoriesConfiguration"

const App = () => {
  return (
    <Sheet>
      <ActivitiesList />
      <PamCategoriesConfiguration />
    </Sheet>
  )
}

export default App
