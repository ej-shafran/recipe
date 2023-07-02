<script lang="ts">
  import { z } from "zod";
  import axios from "axios";
  import { navigate } from "svelte-navigator";
  import { createMutation } from "@tanstack/svelte-query";
  import { getUserId } from "../common/functions/auth.functions";

  import { createForm } from "../common/forms/createForm";
  import Form from "../common/forms/Form.svelte";
  import Field from "../common/forms/Field.svelte";

  const CONFIRM_ERROR = {
    message: "Passwords must match.",
    path: ["confirmPassword"],
  };

  const schema = z
    .object({
      username: z.string().min(5).max(30),
      password: z.string().min(5).max(100),
      confirmPassword: z.string().min(5).max(100),
    })
    .refine((schema) => {
      return schema.password === schema.confirmPassword;
    }, CONFIRM_ERROR);

  const mutation = createMutation({
    mutationKey: ["register"],
    mutationFn: async (values: z.infer<typeof schema>) => {
      await axios.post("/api/user/register", values);
    },
    onSuccess: () => {
      navigate(`/user/${getUserId()}`);
    },
  });

  const form = createForm(schema, mutation);
  const { store } = form;
</script>

<h1>Register Page</h1>
<Form {...form}>
  <Field {store} key={["username"]} label="Enter a username:" />

  <Field {store} key={["password"]} let:attributes>
    <svelte:fragment slot="label">Enter a password:</svelte:fragment>
    <input use:attributes type="password" />
  </Field>

  <Field {store} key={["confirmPassword"]} let:attributes>
    <svelte:fragment slot="label">Confirm your password:</svelte:fragment>
    <input use:attributes type="password" />
  </Field>

  <button type="submit">Register</button>
</Form>
