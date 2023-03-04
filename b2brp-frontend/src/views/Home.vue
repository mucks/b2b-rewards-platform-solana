<template>
  <v-container>
    <h1>You are successfully Authenticated!</h1>
  </v-container>
</template>

<script lang="ts" setup>
import { RequestHandler } from '@/handlers/RequestHandler';
import { onMounted } from 'vue';

onMounted(async () => {
  try {
    const data: string = await RequestHandler.get('/api/test');
    console.log(data);
  } catch (e) {
    console.log(e);
  }
});
</script>
