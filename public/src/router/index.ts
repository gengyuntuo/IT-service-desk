import { createRouter, createWebHistory } from 'vue-router'
import LoginView from '../views/LoginView.vue'
import MainLayout from '../layouts/MainLayout.vue'
import DashboardView from '../views/DashboardView.vue'
import TicketApplyView from '../views/TicketApplyView.vue'
import TicketApproveView from '../views/TicketApproveView.vue'
import AwsAccountView from '../views/AwsAccountView.vue'
import ResourceManagementView from '../views/ResourceManagementView.vue'
import UsersView from '../views/UsersView.vue'
import SettingsView from '../views/SettingsView.vue'

const router = createRouter({
  history: createWebHistory((import.meta as any).env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'login',
      component: LoginView
    },
    {
      path: '/dashboard',
      component: MainLayout,
      children: [
        {
          path: '',
          name: 'dashboard',
          component: DashboardView
        }
      ]
    },
    {
      path: '/ticket-apply',
      component: MainLayout,
      children: [
        {
          path: '',
          name: 'ticket-apply',
          component: TicketApplyView
        }
      ]
    },
    {
      path: '/ticket-approve',
      component: MainLayout,
      children: [
        {
          path: '',
          name: 'ticket-approve',
          component: TicketApproveView
        }
      ]
    },
    {
      path: '/users',
      component: MainLayout,
      children: [
        {
          path: '',
          name: 'users',
          component: UsersView
        }
      ]
    },
    {
      path: '/settings',
      component: MainLayout,
      children: [
        {
          path: '',
          name: 'settings',
          component: SettingsView
        }
      ]
    },
    {
      path: '/aws-account',
      component: MainLayout,
      children: [
        {
          path: '',
          name: 'aws-account',
          component: AwsAccountView
        }
      ]
    },
    {
      path: '/resource-management',
      component: MainLayout,
      children: [
        {
          path: '',
          name: 'resource-management',
          component: ResourceManagementView
        }
      ]
    }
  ]
})

export default router
