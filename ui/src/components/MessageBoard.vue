<template>
  <div>
    <p class="text-center">Message Board</p>
    <post-message v-on:message-posted="fetchMessages" />
    <message-list :messages="messages" />
  </div>
</template>

<script lang="ts">
import axios from "axios";
import { Component, Vue } from "vue-property-decorator";
import PostMessage from "@/components/PostMessage.vue";
import MessageList from "@/components/MessageList.vue";

@Component({
  components: {
    MessageList,
    PostMessage,
  },
})
export default class MessageBoard extends Vue {
  messages: string[] = [];

  mounted(): void {
    this.fetchMessages();
  }

  fetchMessages(): void {
    console.log("Fetching messages");
    axios
      .get("http://127.0.0.1:8000/api/messages")
      .then((response) => {
        this.messages = response.data;
        console.log(this.messages);
      })
      .catch((error) => {
        console.log(error);
      });
  }
}
</script>
