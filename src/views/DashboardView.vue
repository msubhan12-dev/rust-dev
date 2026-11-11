<template>
  <div class="min-h-screen bg-gray-100">
    <!-- Navigation -->
    <nav class="bg-white shadow-lg">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <h1 class="text-xl font-bold text-gray-900">Dashboard</h1>
          </div>
          <div class="flex items-center space-x-4">
            <span class="text-sm text-gray-600">
              {{ authStore.user?.email }}
            </span>
            <button
              @click="handleLogout"
              class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50"
              :disabled="isLoggingOut"
            >
              {{ isLoggingOut ? 'Logging out...' : 'Logout' }}
            </button>
          </div>
        </div>
      </div>
    </nav>

    <!-- Main Content -->
    <div class="max-w-7xl mx-auto py-12 sm:px-6 lg:px-8">
      <!-- Loading State -->
      <div v-if="isLoading" class="flex justify-center py-12">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
      </div>

      <!-- Error State -->
      <div v-else-if="error" class="rounded-md bg-red-50 p-4 mb-8">
        <div class="flex">
          <div class="ml-3">
            <h3 class="text-sm font-medium text-red-800">
              {{ error }}
            </h3>
            <button
              @click="retryLoadUser"
              class="mt-2 text-sm text-red-700 hover:text-red-600 font-medium"
            >
              Retry
            </button>
          </div>
        </div>
      </div>

      <!-- Content -->
      <div v-else class="space-y-8">
        <!-- Welcome Section -->
        <div class="rounded-lg bg-white overflow-hidden shadow-lg">
          <div class="px-4 py-5 sm:p-6">
            <h3 class="text-lg leading-6 font-medium text-gray-900">
              Welcome, {{ authStore.user?.email }}! 👋
            </h3>
            <p class="mt-2 text-sm text-gray-600">
              You are successfully authenticated with JWT. This dashboard is protected and only accessible with a valid token.
            </p>
          </div>
        </div>

        <!-- User Information Card -->
        <div class="rounded-lg bg-white overflow-hidden shadow-lg">
          <div class="px-4 py-5 sm:p-6">
            <h3 class="text-lg leading-6 font-medium text-gray-900 mb-4">
              Account Information
            </h3>
            <dl class="grid grid-cols-1 gap-5 sm:grid-cols-2">
              <div class="px-4 py-5 bg-gray-50 shadow rounded-lg">
                <dt class="text-sm font-medium text-gray-500 truncate">
                  Email Address
                </dt>
                <dd class="mt-1 text-lg font-semibold text-gray-900">
                  {{ authStore.user?.email }}
                </dd>
              </div>
              <div class="px-4 py-5 bg-gray-50 shadow rounded-lg">
                <dt class="text-sm font-medium text-gray-500 truncate">
                  User ID
                </dt>
                <dd class="mt-1 text-xs font-mono text-gray-900 break-all">
                  {{ authStore.user?.id }}
                </dd>
              </div>
              <div class="px-4 py-5 bg-gray-50 shadow rounded-lg">
                <dt class="text-sm font-medium text-gray-500 truncate">
                  Account Created
                </dt>
                <dd class="mt-1 text-sm text-gray-900">
                  {{ formatDate(authStore.user?.created_at) }}
                </dd>
              </div>
              <div class="px-4 py-5 bg-gray-50 shadow rounded-lg">
                <dt class="text-sm font-medium text-gray-500 truncate">
                  Last Updated
                </dt>
                <dd class="mt-1 text-sm text-gray-900">
                  {{ formatDate(authStore.user?.updated_at) }}
                </dd>
              </div>
            </dl>
          </div>
        </div>

        <!-- Authentication Status -->
        <div class="rounded-lg bg-white overflow-hidden shadow-lg">
          <div class="px-4 py-5 sm:p-6">
            <h3 class="text-lg leading-6 font-medium text-gray-900 mb-4">
              Authentication Status
            </h3>
            <div class="space-y-4">
              <div class="flex items-center">
                <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-green-100 text-green-800">
                  ✓ Authenticated
                </span>
              </div>
              <p class="text-sm text-gray-600">
                Your JWT token is valid and automatically injected into all API requests.
              </p>
              <div class="mt-4 p-4 bg-gray-50 rounded-md border border-gray-200">
                <p class="text-xs text-gray-500 font-mono break-all">
                  Token: {{ authStore.token?.substring(0, 50) }}...
                </p>
                <p class="text-xs text-gray-400 mt-2">
                  Token expires in 24 hours from login
                </p>
              </div>
            </div>
          </div>
        </div>

        <!-- Features -->
        <div class="rounded-lg bg-white overflow-hidden shadow-lg">
          <div class="px-4 py-5 sm:p-6">
            <h3 class="text-lg leading-6 font-medium text-gray-900 mb-4">
              🚀 Features Implemented
            </h3>
            <ul class="space-y-3">
              <li class="flex items-start">
                <span class="flex-shrink-0 h-5 w-5 text-green-500 font-bold">✓</span>
                <span class="ml-3 text-sm text-gray-700">
                  <strong>User Registration</strong> - Secure account creation with email and password validation
                </span>
              </li>
              <li class="flex items-start">
                <span class="flex-shrink-0 h-5 w-5 text-green-500 font-bold">✓</span>
                <span class="ml-3 text-sm text-gray-700">
                  <strong>JWT Authentication</strong> - 24-hour token validity with automatic refresh handling
                </span>
              </li>
              <li class="flex items-start">
                <span class="flex-shrink-0 h-5 w-5 text-green-500 font-bold">✓</span>
                <span class="ml-3 text-sm text-gray-700">
                  <strong>Password Security</strong> - Bcrypt hashing with 12 rounds
                </span>
              </li>
              <li class="flex items-start">
                <span class="flex-shrink-0 h-5 w-5 text-green-500 font-bold">✓</span>
                <span class="ml-3 text-sm text-gray-700">
                  <strong>Protected Routes</strong> - Automatic redirects for unauthorized access
                </span>
              </li>
              <li class="flex items-start">
                <span class="flex-shrink-0 h-5 w-5 text-green-500 font-bold">✓</span>
                <span class="ml-3 text-sm text-gray-700">
                  <strong>Auto Token Injection</strong> - JWT automatically added to all API requests
                </span>
              </li>
              <li class="flex items-start">
                <span class="flex-shrink-0 h-5 w-5 text-green-500 font-bold">✓</span>
                <span class="ml-3 text-sm text-gray-700">
                  <strong>Automatic Logout</strong> - Invalid token triggers redirect to login
                </span>
              </li>
              <li class="flex items-start">
                <span class="flex-shrink-0 h-5 w-5 text-green-500 font-bold">✓</span>
                <span class="ml-3 text-sm text-gray-700">
                  <strong>Token Persistence</strong> - Survives page refreshes
                </span>
              </li>
              <li class="flex items-start">
                <span class="flex-shrink-0 h-5 w-5 text-green-500 font-bold">✓</span>
                <span class="ml-3 text-sm text-gray-700">
                  <strong>Responsive Design</strong> - Mobile-friendly UI with Tailwind CSS
                </span>
              </li>
            </ul>
          </div>
        </div>

        <!-- Tech Stack -->
        <div class="rounded-lg bg-white overflow-hidden shadow-lg">
          <div class="px-4 py-5 sm:p-6">
            <h3 class="text-lg leading-6 font-medium text-gray-900 mb-4">
              🛠️ Tech Stack
            </h3>
            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
              <div>
                <h4 class="font-medium text-gray-900 mb-2">Backend</h4>
                <ul class="space-y-1 text-sm text-gray-600">
                  <li>• Rust with Axum</li>
                  <li>• PostgreSQL</li>
                  <li>• JWT Authentication</li>
                  <li>• Bcrypt Password Hashing</li>
                </ul>
              </div>
              <div>
                <h4 class="font-medium text-gray-900 mb-2">Frontend</h4>
                <ul class="space-y-1 text-sm text-gray-600">
                  <li>• Vue.js 3</li>
                  <li>• TypeScript</li>
                  <li>• Pinia State Management</li>
                  <li>• Vue Router</li>
                </ul>
              </div>
            </div>
          </div>
        </div>

        <!-- Next Steps -->
        <div class="rounded-lg bg-blue-50 overflow-hidden shadow-lg border border-blue-200">
          <div class="px-4 py-5 sm:p-6">
            <h3 class="text-lg leading-6 font-medium text-blue-900 mb-4">
              📚 Next Steps
            </h3>
            <ul class="space-y-2 text-sm text-blue-800">
              <li>✓ Phase 1: Backend with JWT authentication</li>
              <li>✓ Phase 2: Frontend with Vue.js</li>
              <li>→ Phase 2.6: Test end-to-end & push to GitHub</li>
              <li>→ Phase 3.1: Setup CORS on backend</li>
              <li>→ Phase 3.2: Integration testing</li>
              <li>→ Phase 3: Final cleanup & deployment</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/authStore'

const router = useRouter()
const authStore = useAuthStore()

const isLoading = ref(false)
const isLoggingOut = ref(false)
const error = ref<string | null>(null)

// Format date helper
const formatDate = (dateString: string | undefined) => {
  if (!dateString) return 'Unknown'
  try {
    const date = new Date(dateString)
    return date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  } catch {
    return dateString
  }
}

// Load user data on mount
const loadUserData = async () => {
  isLoading.value = true
  error.value = null
  try {
    await authStore.getCurrentUser()
  } catch (err: any) {
    error.value = err.response?.data?.message || 'Failed to load user data'
    console.error('Failed to load user data:', err)
  } finally {
    isLoading.value = false
  }
}

// Retry loading user data
const retryLoadUser = () => {
  loadUserData()
}

// Handle logout
const handleLogout = async () => {
  isLoggingOut.value = true
  try {
    await authStore.logout()
    router.push('/login')
  } catch (error) {
    console.error('Logout failed:', error)
    // Force redirect even if logout fails
    router.push('/login')
  } finally {
    isLoggingOut.value = false
  }
}

// Load user data when component mounts
onMounted(() => {
  // User is already loaded from store/localStorage during auth
  // This would fetch fresh data from backend if needed
  if (!authStore.user) {
    loadUserData()
  }
})
</script>
