import axios, { AxiosInstance, AxiosError, AxiosResponse } from 'axios'

/**
 * Axios HTTP Client Instance
 * Configured with base URL, timeout, and interceptors
 */

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'
const TIMEOUT = import.meta.env.VITE_API_TIMEOUT || 10000

export const apiClient: AxiosInstance = axios.create({
  baseURL: API_URL,
  timeout: TIMEOUT,
  headers: {
    'Content-Type': 'application/json',
  },
})

/**
 * Request Interceptor
 * Adds JWT token to Authorization header
 */
apiClient.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('auth_token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

/**
 * Response Interceptor
 * Handles token expiration and errors
 */
apiClient.interceptors.response.use(
  (response) => response,
  (error: AxiosError) => {
    // Handle 401 Unauthorized - token expired
    if (error.response?.status === 401) {
      localStorage.removeItem('auth_token')
      localStorage.removeItem('auth_user')
      // Redirect to login (will be handled by router guard)
      window.location.href = '/login'
    }

    return Promise.reject(error)
  }
)

export default apiClient
