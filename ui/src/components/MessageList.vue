<template>
  <ul>
    <li
      v-for="(message, index) in messages"
      :key="index"
      class="p-4 my-2 rounded-lg bg-blue-400"
    >
      <message :message="message" />
    </li>
  </ul>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import Message from "@/components/Message.vue";

@Component({
  components: {
    Message,
  },
})
export default class MessageBoard extends Vue {
  @Prop({
    default: () => [],
  })
  messages!: string[];
}
</script>
