<template>
  <div>
    <form class="text-center">
      <label for="message" class="mx-2">Message:</label>
      <input type="text" id="message" v-model.trim="message" />
      <button
        type="button"
        class="p-1 mx-2 border-4 rounded-lg border-blue-500 bg-blue-300"
        @click="postMessage"
      >
        Post
      </button>
    </form>
  </div>
</template>

<script lang="ts">
import axios from "axios";
import { Component, Emit, Vue } from "vue-property-decorator";

@Component
export default class MessageBoard extends Vue {
  message = "";

  @Emit("message-posted")
  postMessage(): void {
    axios
      .post("http://127.0.0.1:8000/api/messages", this.message)
      .then(() => {
        this.message = "";
      })
      .catch((error) => {
        console.log(error);
      });
  }
}
</script>
