<script lang="ts">
  import { z } from "zod";
  import axios from "axios";
  import type { AxiosError } from "axios";
  import { navigate } from "svelte-navigator";
  import { createMutation } from "@tanstack/svelte-query";
  import { getUserId } from "../common/functions/auth.functions";

  import { createForm } from "../common/forms/createForm";
  import Form from "../common/forms/Form.svelte";
  import Field from "../common/forms/Field.svelte";
  import SubmitButton from "../common/forms/SubmitButton.svelte";
  import { schemas } from "../common/forms/schemas";
  import { errors } from "../common/forms/errors";

  const mutation = createMutation({
    mutationKey: ["login"],
    mutationFn: async (values: z.infer<typeof schemas.credentials>) => {
      await axios.post("/api/user/login", values);
    },
    onSuccess: () => {
      navigate(`/user/${getUserId()}`);
    },
    onError: (error: AxiosError) => {
      if (error.response?.status === 401) {
        $store.errors = {
          username: { _errors: [errors.invalidCredentials()] },
        } as any;
      }
    },
  });

  const form = createForm(schemas.credentials, mutation);
  const { store } = form;
</script>

<h1>Login Page</h1>
<Form {...form} data-cy="LOGIN_FORM">
  <Field {store} key={["username"]} label="Enter your username:" />

  <Field {store} key={["password"]} label="Enter your password:" let:attributes>
    <input type="password" use:attributes />
  </Field>

  <SubmitButton>Log In</SubmitButton>
</Form>
