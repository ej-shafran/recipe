<script lang="ts">
  import axios from "axios";
  import { navigate } from "svelte-navigator";
  import { createMutation } from "@tanstack/svelte-query";
  import { getUserId } from "../common/functions/auth.functions";

  import { z } from "zod";
  import { createForm } from "../common/forms/createForm";
  import Form from "../common/forms/Form.svelte";
  import Field from "../common/forms/Field.svelte";

  const schema = z.object({
    username: z.string().min(5).max(30),
    password: z.string().min(5).max(100),
  });

  const mutation = createMutation({
    mutationKey: ["login"],
    mutationFn: async (values: z.infer<typeof schema>) => {
      await axios.post("/api/user/login", values);
    },
    onSuccess: () => {
      navigate(`/user/${getUserId()}`);
    },
  });

  const form = createForm(schema, mutation);
  const { store } = form;
</script>

<h1>Login Page</h1>
<Form {...form}>
  <Field {store} key={["username"]} label="Enter your username:" />

  <Field {store} key={["password"]} label="Enter your password:" let:attributes>
    <input type="password" use:attributes />
  </Field>

  <button type="submit">Log In</button>
</Form>
