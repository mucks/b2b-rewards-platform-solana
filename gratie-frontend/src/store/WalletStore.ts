import { defineStore } from "pinia";

export const useWalletStore = defineStore("WalletStore", {
  state: () => ({
    // Public key of the wallet
    wallet: "",
  }),
  getters: {
    isWalletConnected(state: any): boolean {
      return state.wallet != "";
    }
  },
  actions: {
    setWallet(wallet: string) {
      this.wallet = wallet;
    }
  }
});