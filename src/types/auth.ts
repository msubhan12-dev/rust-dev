/**
 * Authentication Types & Interfaces
 */

export interface User {
  id: string;
  email: string;
  created_at: string;
  updated_at: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  user: User;
  token: string;
  expires_in: number;
}

export interface RegisterRequest {
  email: string;
  password: string;
  confirm_password: string;
}

export interface RegisterResponse {
  user: User;
  message: string;
}

export interface ApiError {
  error: string;
  message: string;
}

export interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
}
