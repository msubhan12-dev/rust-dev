import { apiClient } from './client'
import type { LoginRequest, LoginResponse, RegisterRequest, RegisterResponse } from '@/types/auth'

/**
 * Authentication API Service
 * Handles all authentication-related API calls
 */

export const authApi = {
  /**
   * Register a new user
   */
  register: async (data: RegisterRequest): Promise<RegisterResponse> => {
    const response = await apiClient.post<RegisterResponse>('/api/auth/register', data)
    return response.data
  },

  /**
   * Login user and get JWT token
   */
  login: async (data: LoginRequest): Promise<LoginResponse> => {
    const response = await apiClient.post<LoginResponse>('/api/auth/login', data)
    return response.data
  },

  /**
   * Logout user
   */
  logout: async (): Promise<void> => {
    await apiClient.post('/api/auth/logout')
  },

  /**
   * Get current authenticated user
   * Requires valid JWT token
   */
  getCurrentUser: async () => {
    const response = await apiClient.get('/api/users/me')
    return response.data
  },
}

export default authApi
