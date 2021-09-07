import LandingPage from "../pages/Landing";
import Dashboard from "../pages/Dashboard";

const routes = [
    {
        label: 'home',
        path: '/',
        exact: true,
        component: LandingPage    
    },
    {
        label: 'dashboard',
        path: '/dashboard/:principalId',
        exact: true,
        component: Dashboard         
    }
]

export default routes;