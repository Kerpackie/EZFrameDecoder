<template>
  <decoded-view
      :result="result"
      :error="error"
      class="flex-1"
  />
</template>

<script setup lang="ts">
import { useSharedDecode } from "../composables/useSharedDecode";
import DecodedView from "../components/DecodedView.vue";

/* The shared composable holds reactive result & error */
const { result, error } = useSharedDecode();
</script>
