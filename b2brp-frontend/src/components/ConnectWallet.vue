<template>
  <div class="mx-2">
    <v-btn v-if="wallet" variant="flat" color="success" :disabled="true">{{ walletShort }}</v-btn>
    <v-btn variant="flat" v-else @click="connectWallet">Connect Wallet</v-btn>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref } from 'vue';

const wallet = ref(null as null | string);

const walletShort = computed(() => {
  return wallet.value?.substring(1, 4) + '...' + wallet.value?.substring(wallet.value.length - 4, wallet.value.length);
});


onMounted(async () => {
  nextTick(async () => {
    await checkWallet();
  });
});

const connectWallet = async () => {
  const { solana } = window as any;
  if (solana) {
    const response = await solana.connect();
    console.log("connected with public key");
    wallet.value = response.publicKey.toString();
  }
}

const checkWallet = async () => {
  try {
    const { solana } = window as any;
    if (solana && solana.isPhantom) {
      console.log('Phantom wallet is installed');
      const response = await solana.connect({ onlyIfTrusted: true });
      console.log('connected with public key:', response.publicKey.toString());
      wallet.value = response.publicKey.toString();
    } else {
      console.log('Phantom wallet is not installed');
    }
  } catch (e) {
    console.log(e);
  }
}


</script>

<style scoped></style>