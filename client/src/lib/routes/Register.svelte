<script lang="ts">
  import axios from "axios";
  import { navigate } from "svelte-navigator";
  import { createMutation } from "@tanstack/svelte-query";
  import UserForm from "../components/UserForm.svelte";
  import { getUserId } from "../common/functions/auth.functions";
  import type { UserValues } from "../common/schemas/user.schema";

  const mutation = createMutation({
    mutationKey: ["register"],
    mutationFn: async (values: UserValues) => {
      await axios.post("/api/user/register", values);
    },
    onSuccess: () => {
      navigate(`/user/${getUserId()}`);
    },
  });
</script>

<h1>Register Page</h1>
<UserForm submit={$mutation.mutate} buttonText="Register" />
