<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <!-- Header -->
      <div>
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
          Create your account
        </h2>
        <p class="mt-2 text-center text-sm text-gray-600">
          Or
          <RouterLink to="/login" class="font-medium text-indigo-600 hover:text-indigo-500">
            sign in to existing account
          </RouterLink>
        </p>
      </div>

      <!-- Form -->
      <form class="mt-8 space-y-6" @submit.prevent="handleRegister">
        <!-- Error Alert -->
        <div
          v-if="authStore.error"
          class="rounded-md bg-red-50 p-4"
        >
          <div class="flex">
            <div class="ml-3">
              <h3 class="text-sm font-medium text-red-800">
                {{ authStore.error }}
              </h3>
            </div>
          </div>
        </div>

        <!-- Success Alert -->
        <div
          v-if="successMessage"
          class="rounded-md bg-green-50 p-4"
        >
          <div class="flex">
            <div class="ml-3">
              <h3 class="text-sm font-medium text-green-800">
                {{ successMessage }}
              </h3>
            </div>
          </div>
        </div>

        <!-- Email Field -->
        <div>
          <label for="email" class="block text-sm font-medium text-gray-700">
            Email address
          </label>
          <input
            id="email"
            v-model="form.email"
            type="email"
            required
            class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
            placeholder="user@example.com"
          />
        </div>

        <!-- Password Field -->
        <div>
          <label for="password" class="block text-sm font-medium text-gray-700">
            Password
          </label>
          <input
            id="password"
            v-model="form.password"
            type="password"
            required
            class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
            placeholder="••••••••"
          />
          <p class="mt-1 text-xs text-gray-500">
            Minimum 8 characters
          </p>
        </div>

        <!-- Confirm Password Field -->
        <div>
          <label for="confirm-password" class="block text-sm font-medium text-gray-700">
            Confirm password
          </label>
          <input
            id="confirm-password"
            v-model="form.confirm_password"
            type="password"
            required
            class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
            placeholder="••••••••"
          />
        </div>

        <!-- Validation Error -->
        <div v-if="validationError" class="rounded-md bg-yellow-50 p-4">
          <div class="flex">
            <div class="ml-3">
              <h3 class="text-sm font-medium text-yellow-800">
                {{ validationError }}
              </h3>
            </div>
          </div>
        </div>

        <!-- Submit Button -->
        <button
          type="submit"
          :disabled="authStore.isLoading"
          class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {{ authStore.isLoading ? 'Creating account...' : 'Create account' }}
        </button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { RouterLink, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/authStore'

const router = useRouter()
const authStore = useAuthStore()

const form = reactive({
  email: '',
  password: '',
  confirm_password: '',
})

const successMessage = ref('')
const validationError = ref('')

const handleRegister = async () => {
  // Clear previous errors
  validationError.value = ''
  authStore.clearError()

  // Validate passwords match
  if (form.password !== form.confirm_password) {
    validationError.value = 'Passwords do not match'
    return
  }

  // Validate password length
  if (form.password.length < 8) {
    validationError.value = 'Password must be at least 8 characters'
    return
  }

  try {
    await authStore.register({
      email: form.email,
      password: form.password,
      confirm_password: form.confirm_password,
    })

    successMessage.value = 'Account created successfully! Redirecting to login...'

    // Redirect to login after 2 seconds
    setTimeout(() => {
      router.push('/login')
    }, 2000)
  } catch (error) {
    // Error is handled by store and displayed in template
    console.error('Registration failed:', error)
  }
}
</script>
