<template>
  <div>
    <create-company-license v-if="store.isWalletConnected" />

  </div>
</template>

<script lang="ts" setup>
import { useWalletStore } from '@/store/WalletStore';
import { onMounted } from 'vue';
import CreateCompanyLicense from '@/components/CreateCompanyLicense.vue';

const store = useWalletStore();

onMounted(async () => {
});

</script>
