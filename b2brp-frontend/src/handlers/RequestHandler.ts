import { useAuthStore } from "@/stores/AuthStore";
import axios from "axios";

export class RequestHandler {
  private static axios() {
    const store = useAuthStore();

    return axios.create({
      baseURL: process.env.NODE_ENV === "development" ? "http://localhost:4000" : "https://b2brp-api.mucks.dev",
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer ' + store.token,
      },

    });
  }

  public static async get<T>(url: string): Promise<T> {
    const response = await this.axios().get(url);
    return response.data;
  }
  public static async post<T>(url: string, data: any): Promise<T> {
    const response = await this.axios().post(url, data);
    return response.data;
  }
}