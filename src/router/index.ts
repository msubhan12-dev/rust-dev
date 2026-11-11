import { createRouter, createWebHistory, RouteRecordRaw, NavigationGuardNext, RouteLocationNormalized } from 'vue-router'
import { useAuthStore } from '@/stores/authStore'

// View imports
import LoginView from '@/views/LoginView.vue'
import RegisterView from '@/views/RegisterView.vue'
import DashboardView from '@/views/DashboardView.vue'
import NotFoundView from '@/views/NotFoundView.vue'

/**
 * Route definitions
 */
const routes: RouteRecordRaw[] = [
  {
    path: '/',
    redirect: () => {
      const authStore = useAuthStore()
      return authStore.isAuthenticated ? '/dashboard' : '/login'
    },
  },
  {
    path: '/login',
    name: 'login',
    component: LoginView,
    meta: {
      requiresAuth: false,
      title: 'Sign In',
    },
  },
  {
    path: '/register',
    name: 'register',
    component: RegisterView,
    meta: {
      requiresAuth: false,
      title: 'Create Account',
    },
  },
  {
    path: '/dashboard',
    name: 'dashboard',
    component: DashboardView,
    meta: {
      requiresAuth: true,
      title: 'Dashboard',
    },
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'notFound',
    component: NotFoundView,
    meta: {
      title: 'Not Found',
    },
  },
]

/**
 * Create router instance
 */
export const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

/**
 * Navigation guards - Check authentication before navigation
 */
router.beforeEach(
  async (
    to: RouteLocationNormalized,
    from: RouteLocationNormalized,
    next: NavigationGuardNext
  ) => {
    const authStore = useAuthStore()

    // Update page title
    document.title = `${to.meta.title || 'Home'} - Auth App`

    // Check if route requires authentication
    const requiresAuth = to.meta.requiresAuth as boolean | undefined

    if (requiresAuth) {
      // Route requires authentication
      if (authStore.isAuthenticated) {
        // User is authenticated, allow navigation
        next()
      } else {
        // User not authenticated, redirect to login
        console.log('Redirecting to login - not authenticated')
        next('/login')
      }
    } else if (to.path === '/login' || to.path === '/register') {
      // Auth pages - redirect authenticated users to dashboard
      if (authStore.isAuthenticated) {
        next('/dashboard')
      } else {
        next()
      }
    } else {
      // No auth requirement, allow navigation
      next()
    }
  }
)

/**
 * After navigation hook - Log navigation for debugging
 */
router.afterEach((to, from) => {
  console.log(`Navigated from ${from.path} to ${to.path}`)
})

export default router
