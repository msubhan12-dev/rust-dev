<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full text-center space-y-8">
      <div>
        <h1 class="text-6xl font-extrabold text-gray-900">
          404
        </h1>
        <p class="mt-4 text-xl font-semibold text-gray-700">
          Page not found
        </p>
        <p class="mt-2 text-sm text-gray-600">
          The page you are looking for does not exist.
        </p>
      </div>

      <RouterLink
        to="/"
        class="inline-flex items-center px-4 py-2 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700"
      >
        Go home
      </RouterLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router'
</script>
