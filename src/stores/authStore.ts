import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi } from '@/api/auth'
import type { User, LoginRequest, RegisterRequest } from '@/types/auth'

/**
 * Authentication Store
 * Manages auth state, token, and user data
 */

export const useAuthStore = defineStore('auth', () => {
  // State
  const user = ref<User | null>(null)
  const token = ref<string | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Load token from localStorage on store initialization
  const savedToken = localStorage.getItem('auth_token')
  if (savedToken) {
    token.value = savedToken
  }

  const savedUser = localStorage.getItem('auth_user')
  if (savedUser) {
    try {
      user.value = JSON.parse(savedUser)
    } catch (e) {
      console.error('Failed to parse saved user', e)
    }
  }

  // Computed
  const isAuthenticated = computed(() => !!token.value && !!user.value)

  // Actions
  const login = async (credentials: LoginRequest) => {
    isLoading.value = true
    error.value = null

    try {
      const response = await authApi.login(credentials)

      // Save token and user
      token.value = response.token
      user.value = response.user

      // Persist to localStorage
      localStorage.setItem('auth_token', response.token)
      localStorage.setItem('auth_user', JSON.stringify(response.user))

      return response
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Login failed'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const register = async (data: RegisterRequest) => {
    isLoading.value = true
    error.value = null

    try {
      const response = await authApi.register(data)
      return response
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Registration failed'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const logout = async () => {
    try {
      await authApi.logout()
    } catch (err) {
      console.error('Logout error:', err)
    } finally {
      // Clear state regardless of API response
      token.value = null
      user.value = null
      error.value = null

      // Clear localStorage
      localStorage.removeItem('auth_token')
      localStorage.removeItem('auth_user')
    }
  }

  const getCurrentUser = async () => {
    isLoading.value = true
    error.value = null

    try {
      const userData = await authApi.getCurrentUser()
      user.value = userData
      localStorage.setItem('auth_user', JSON.stringify(userData))
      return userData
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Failed to get user'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const clearError = () => {
    error.value = null
  }

  return {
    // State
    user,
    token,
    isLoading,
    error,

    // Computed
    isAuthenticated,

    // Actions
    login,
    register,
    logout,
    getCurrentUser,
    clearError,
  }
})
