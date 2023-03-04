<template>
  <router-view />
</template>

<script lang="ts" setup>
import { useAuthStore } from '@/stores/AuthStore';
import { useRouter } from 'vue-router';
import { onMounted } from 'vue';
import { RequestHandler } from './handlers/RequestHandler';

const auth = useAuthStore();
const router = useRouter();

const checkAuth = async () => {
  try {
    await RequestHandler.get('/api/test');
  } catch (e) {
    auth.setToken('');
    router.push('/login');
  }
}

onMounted(async () => {
  await checkAuth();
});
</script>
