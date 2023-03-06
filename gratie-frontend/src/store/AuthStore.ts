import { defineStore } from "pinia";

export const useAuthStore = defineStore("AuthStore", {
  state: () => ({
    token: localStorage.getItem("token") || "",
    isLoggedIn: localStorage.getItem("token") ? true : false,
  }),
  actions: {
    setToken(token: string) {
      this.token = token;
      this.isLoggedIn = token != '';
      localStorage.setItem("token", token);
    }
  }
});