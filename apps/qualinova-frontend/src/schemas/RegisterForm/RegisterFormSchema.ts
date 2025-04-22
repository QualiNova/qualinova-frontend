import { z } from "zod";

export const registerFormSchema = z.object({
  firstName: z.string().min(2, "First name must be at least 2 characters"),
  lastName: z.string().min(2, "Last name must be at least 2 characters"),
  email: z.string().email("Invalid email format"),
  password: z.string().min(8, "Password must be at least 8 characters long"),
  terms: z.literal(true, {
    errorMap: () => ({ message: "You must accept the terms" }),
  }),
});
