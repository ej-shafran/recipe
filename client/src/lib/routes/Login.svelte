<script lang="ts">
  import axios from "axios";
  import { navigate } from "svelte-navigator";
  import { createMutation } from "@tanstack/svelte-query";
  import UserForm from "../components/UserForm.svelte";
  import { getUserId } from "../common/functions/auth.functions";
  import type { UserValues } from "../common/schemas/user.schema";

  const mutation = createMutation({
    mutationKey: ["login"],
    mutationFn: async (values: UserValues) => {
      await axios.post("/api/login", values);
    },
    onSuccess: () => {
      navigate(`/user/${getUserId()}`);
    },
  });
</script>

<h1>Login Page</h1>
<UserForm submit={$mutation.mutate} buttonText="Log In" />
