<template>
  <div>
    <h1>Login</h1>
    <v-form @submit.prevent="submit">
      <v-text-field type="email" label="Email" v-model="dto.email" />
      <v-text-field type="password" label="Password" v-model="dto.password" />
      <v-btn type="submit">Submit</v-btn>
    </v-form>

  </div>
</template>

<script setup lang="ts">
import { RequestHandler } from '@/handlers/RequestHandler';
import { reactive } from 'vue';
import { useAuthStore } from '@/stores/AuthStore';
import { useRouter } from 'vue-router';

const auth = useAuthStore();
const router = useRouter();

const dto = reactive({
  email: '',
  password: '',
});

const submit = async (event) => {
  try {
    const token: string = await RequestHandler.post('/public-api/login', dto);
    auth.setToken(token);
    router.push('/');
  } catch (e) {
    console.log(e);
  }


};


</script>

<style scoped></style>