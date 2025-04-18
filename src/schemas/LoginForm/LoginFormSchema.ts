import { z } from "zod";

export const loginFormSchema = z.object({
 email: z.string().email("Invalid email format"),
  password: z.string().min(8, "Password must be at least 8 characters long"),
  rememberMe: z.boolean().optional(),
});