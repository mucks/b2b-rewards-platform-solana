<template>
  <v-app-bar flat>
    <v-app-bar-title>
      <v-img width="150px" src="@/assets/logo.png" />
    </v-app-bar-title>
    <v-spacer />
    <!-- <v-btn to="/sign-up">Sign Up</v-btn>
    <v-btn to="/login">Login</v-btn> -->
    <connect-wallet />
  </v-app-bar>
</template>

<script lang="ts" setup>
import ConnectWallet from '@/components/ConnectWallet.vue';
  //
</script>
