<script lang="ts">
  import axios from "axios";
  import UserForm from "../components/UserForm.svelte";
  import type { UserValues } from "../common/schemas/user.schema";
  import { navigate } from "svelte-navigator";
  import { getUserId } from "../common/functions/auth.functions";

  async function handleSubmit(values: UserValues) {
    try {
      await axios.post("/api/register", values);
      navigate(`/user/${getUserId()}`);
    } catch (error) {
      //TODO: handle errors with a pop-up?
    }
  }
</script>

<h1>Register Page</h1>
<UserForm submit={handleSubmit} buttonText="Register" />
