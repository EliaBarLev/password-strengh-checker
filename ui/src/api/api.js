import axios from "axios";

export async function scorePassword(password) {
  return axios.post("/score", { password });
}
