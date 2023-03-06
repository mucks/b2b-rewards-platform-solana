<template>
  <v-container>
    <v-row justify="center">
      <v-col md="3">
        <h1>Sign Up</h1>
        <v-form @submit.prevent="submit">
          <v-text-field label="Name" v-model="dto.name" />
          <v-text-field type="email" label="Email" v-model="dto.email" />
          <v-text-field type="password" label="Password" v-model="dto.password" />
          <v-btn type="submit">Submit</v-btn>
        </v-form>
      </v-col>
    </v-row>
  </v-container>
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
  name: '',
  password: '',
});

const submit = async (event) => {
  try {
    const token: string = await RequestHandler.post('/public-api/sign-up', dto);
    auth.setToken(token);
    router.push('/');
  } catch (e) {
    console.log(e);
  }
}

</script>

<style scoped></style>