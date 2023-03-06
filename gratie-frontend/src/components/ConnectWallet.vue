<template>
  <div class="mx-2">
    <v-btn v-if="store.isWalletConnected" variant="flat" color="success" :disabled="true">{{ walletShort
    }}</v-btn>
    <v-btn variant="flat" v-else @click="connectWallet">Connect Wallet</v-btn>
  </div>
</template>

<script setup lang="ts">
import { useWalletStore } from '@/store/WalletStore';
import { computed, nextTick, onMounted, ref } from 'vue';

const store = useWalletStore();


const walletShort = computed(() => {
  const len = store.wallet.length;
  return store.wallet?.substring(1, 4) + '...' + store.wallet?.substring(len - 4, len);
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
    store.setWallet(response.publicKey.toString());
  }
}

const checkWallet = async () => {
  try {
    const { solana } = window as any;
    if (solana && solana.isPhantom) {
      console.log('Phantom wallet is installed');
      const response = await solana.connect({ onlyIfTrusted: true });
      console.log('connected with public key:', response.publicKey.toString());
      store.setWallet(response.publicKey.toString());
    } else {
      console.log('Phantom wallet is not installed');
    }
  } catch (e) {
    console.log(e);
  }
}


</script>

<style scoped></style>