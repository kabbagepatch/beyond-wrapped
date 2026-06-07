import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface User {
  id: string
  display_name: string
  email: string
  createdAt: Date
}

export const useUserStore = defineStore('user', () => {
  const user = ref<User | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const setUser = (userData: User) => {
    user.value = userData
    error.value = null
  }

  const clearUser = () => {
    user.value = null
    error.value = null
  }

  const setLoading = (loading: boolean) => {
    isLoading.value = loading
  }

  const setError = (errorMessage: string | null) => {
    error.value = errorMessage
  }

  return {
    user,
    isLoading,
    error,
    setUser,
    clearUser,
    setLoading,
    setError
  }
})