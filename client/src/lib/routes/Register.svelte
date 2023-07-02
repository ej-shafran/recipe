<script lang="ts">
  import { z } from "zod";
  import axios from "axios";
  import { navigate } from "svelte-navigator";
  import { createMutation } from "@tanstack/svelte-query";
  import { getUserId } from "../common/functions/auth.functions";

  import { createForm } from "../common/forms/createForm";
  import Form from "../common/forms/Form.svelte";
  import Field from "../common/forms/Field.svelte";
  import SubmitButton from "../common/forms/SubmitButton.svelte";
  import { schemas } from "../common/forms/schemas";

  const mutation = createMutation({
    mutationKey: ["register"],
    mutationFn: async (values: z.infer<typeof schemas.register>) => {
      await axios.post("/api/user/register", values);
    },
    onSuccess: () => {
      navigate(`/user/${getUserId()}`);
    },
  });

  const form = createForm(schemas.register, mutation);
  const { store } = form;
</script>

<h1>Register Page</h1>
<Form {...form} data-cy="REGISTER_FORM">
  <Field {store} key={["username"]} label="Enter a username:" />

  <Field {store} key={["password"]} let:attributes>
    <svelte:fragment slot="label">Enter a password:</svelte:fragment>
    <input use:attributes type="password" />
  </Field>

  <Field {store} key={["confirmPassword"]} let:attributes>
    <svelte:fragment slot="label">Confirm your password:</svelte:fragment>
    <input use:attributes type="password" />
  </Field>

  <SubmitButton>Register</SubmitButton>
</Form>
