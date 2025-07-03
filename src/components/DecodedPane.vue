<template>
  <decoded-view class="scroll-hide" :result="decoded" :error="error" />
</template>

<script setup lang="ts">
import { useSharedDecode } from "../composables/useSharedDecode";
import DecodedView from "./DecodedView.vue";

const { decoded, error } = useSharedDecode();
</script>
