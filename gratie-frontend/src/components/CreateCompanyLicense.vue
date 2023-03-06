<template>
  <v-container>
    <v-row justify="center">
      <v-col md="4">
        <v-card>
          <v-card-title>
            <h1 class="display-1">Create Company License</h1>
          </v-card-title>
          <v-card-text>
            <v-form @submit.prevent="createCompanyLicense">
              <v-text-field label="Company Name" v-model="companyName" />
              <v-btn type="submit" color="primary">Create</v-btn>
            </v-form>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import { GratieCompanyLicenseNftHandler } from '@/handlers/GratieCompanyLicenseNftHandler';
import { ref } from 'vue';

const companyName = ref('');

const createCompanyLicense = async () => {
  try {
    const resp = await GratieCompanyLicenseNftHandler.createCompanyLicenseNft(companyName.value);
    console.log(resp);
  } catch (e) {
    console.log(e);
  }
}

</script>

<style scoped></style>