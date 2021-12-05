<template>
  <p>{{ message }}</p>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";

@Component
export default class MessageBoard extends Vue {
  @Prop({
    default: "",
  })
  readonly message!: string;
}
</script>
